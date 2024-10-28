use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use url::Url;

use crate::domain::{Article, NewsSource};

#[tracing::instrument("Scrape irish times articles")]
pub async fn scrape_latest_articles(
    http_client: &Client,
    url: Url,
    tag: String,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let response = http_client
        .get(url.clone())
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(url, &document, tag)?;

    Ok(articles)
}

fn parse_articles(url: Url, document: &Html, tag: String) -> Result<Vec<Article>, Box<dyn Error>> {
    let selector = Selector::parse("article")?;
    let articles = document
        .select(&selector)
        .map(|article| {
            let (title, link) = parse_title_and_link(&article);
            let description = parse_description(&article);

            let mut url = url.clone();
            url.set_path(link.as_str());

            Article::new(
                title,
                description,
                url,
                NewsSource::IrishTimes,
                vec![tag.clone()].into(),
            )
            .unwrap() // TODO: Handle error and skip broken articles
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
    use url::Url;

    use crate::domain::{Article, NewsSource, Tag, Tags};

    use super::parse_articles;

    #[rstest]
    #[case(
        r#"<body><div><article><div><h2><a href="/path/to/article">Smart heater gives greater control over comfort and cost</a></h2><p><a>Tech review: Aeno Premium Eco Smart Heater</a></p></div></article></div></body>"#,
        Article::new(
            String::from("Smart heater gives greater control over comfort and cost"),
            Some(String::from("Tech review: Aeno Premium Eco Smart Heater")),
            Url::parse("https://irishtimes.com/path/to/article").unwrap(),
            NewsSource::IrishTimes,
            Tags(vec![Tag::new(String::from("Technology")).unwrap()]),
        ).unwrap()
    )]
    #[case(
        r#"<body><div><article><div><h2><a href="path/to/article">Smart heater gives greater control over comfort and cost</a></h2></div></article></div></body>"#,
        Article::new(
            String::from("Smart heater gives greater control over comfort and cost"),
            None,
            Url::parse("https://irishtimes.com/path/to/article").unwrap(),
            NewsSource::IrishTimes,
            Tags(vec![Tag::new(String::from("Technology")).unwrap()]),
        ).unwrap()
    )]
    fn parse_article_correctly(#[case] html: String, #[case] expected: Article) {
        let tag = String::from("Technology");
        let url = Url::parse("https://irishtimes.com").unwrap();
        let actual = parse_articles(url, &Html::parse_fragment(&html), tag).unwrap();

        assert_eq!(actual, vec![expected]);
    }
}
