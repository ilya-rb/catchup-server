use std::error::Error;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::domain::{Article, NewsSource, uuid_factory};

#[tracing::instrument("Scrape irish times articles")]
pub async fn scrape_latest_articles(
    http_client: &Client,
    url: String,
    tag: String,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(&document, tag)?;

    Ok(articles)
}

fn parse_articles(document: &Html, tag: String) -> Result<Vec<Article>, Box<dyn Error>> {
    let selector = Selector::parse("article")?;
    let articles = document
        .select(&selector)
        .map(|article| {
            let (title, link) = parse_title_and_link(&article);
            let description = parse_description(&article);

            Article {
                id: uuid_factory::new_uuid(),
                source: NewsSource::IrishTimes,
                link,
                title,
                description,
                tags: Some(vec![tag.clone()])
            }
        })
        .collect::<Vec<Article>>();

    Ok(articles)
}

fn parse_title_and_link(article: &ElementRef) -> (String, String) {
    let title_selector = Selector::parse("h2 a").unwrap();
    let title_element = article.select(&title_selector).next().unwrap();

    let title = title_element
        .text()
        .next()
        .expect("Cannot find text for <a> link");

    let link = title_element
        .value()
        .attr("href")
        .expect("Cannot find a link for <a> tag");

    (title.into(), link.into())
}

fn parse_description(article: &ElementRef) -> Option<String> {
    let description_selector = Selector::parse("p a").unwrap();
    let description_element = article.select(&description_selector).next();

    description_element.map(|d| d.text().next().unwrap().into())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use scraper::Html;

    use crate::domain::Article;
    use crate::domain::NewsSource;
    use crate::domain::uuid_factory;

    use super::parse_articles;

    #[rstest]
    #[case(
        r#"<body><div><article><div><h2><a href="/technology/consumer-tech/review/2024/05/02/smart-heater-gives-greater-control-over-comfort-and-cost/">Smart heater gives greater control over comfort and cost</a></h2><p><a>Tech review: Aeno Premium Eco Smart Heater</a></p></div></article></div></body>"#,
        Article {
            id: uuid_factory::new_uuid(),
            title: String::from("Smart heater gives greater control over comfort and cost"),
            description: Some(String::from("Tech review: Aeno Premium Eco Smart Heater")),
            link: String::from("/technology/consumer-tech/review/2024/05/02/smart-heater-gives-greater-control-over-comfort-and-cost/"),
            source: NewsSource::IrishTimes,
            tags: Some(vec![String::from("Technology")]),
        }
    )]
    #[case(
        r#"<body><div><article><div><h2><a href="/technology/consumer-tech/review/2024/05/02/smart-heater-gives-greater-control-over-comfort-and-cost/">Smart heater gives greater control over comfort and cost</a></h2></div></article></div></body>"#,
        Article {
            id: uuid_factory::new_uuid(),
            title: String::from("Smart heater gives greater control over comfort and cost"),
            description: None,
            link: String::from("/technology/consumer-tech/review/2024/05/02/smart-heater-gives-greater-control-over-comfort-and-cost/"),
            source: NewsSource::IrishTimes,
            tags: Some(vec![String::from("Technology")]),
        }
    )]
    fn parse_article_correctly(#[case] html: String, #[case] expected: Article) {
        let tag = String::from("Technology");
        let actual = parse_articles(&Html::parse_fragment(&html), tag).unwrap();

        assert_eq!(actual, vec![expected]);
    }
}
