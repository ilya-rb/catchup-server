use std::error::Error;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::domain::{Article, NewsSource};

#[tracing::instrument("Scraping DOU articles")]
pub async fn scrape_latest_articles(
    http_client: &Client,
    url: String,
) -> Result<Vec<Article>, Box<dyn Error>> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(&document)?;

    Ok(articles)
}

fn parse_articles(document: &Html) -> Result<Vec<Article>, Box<dyn Error>> {
    let selector = Selector::parse("div.b-lenta article")?;
    let articles = document
        .select(&selector)
        .map(|article| {
            let (title, link) = parse_title_and_link(&article);
            let description = parse_description(&article);
            let tags = parse_tags(&article);

            Article::new(title, Some(description), link, NewsSource::Dou, tags)
        })
        .collect::<Vec<Article>>();

    Ok(articles)
}

fn parse_title_and_link(element: &ElementRef) -> (String, String) {
    let selector = Selector::parse("h2 a").unwrap();
    let element = element.select(&selector).next().unwrap();

    let title = element.text().next().expect("Cannot find text for title");

    let link = element
        .value()
        .attr("href")
        .expect("Cannot find a link for title");

    (title.into(), link.into())
}

fn parse_description(element: &ElementRef) -> String {
    let selector = Selector::parse("p").unwrap();
    let element = element.select(&selector).next();

    element.unwrap().text().next().unwrap().into()
}

fn parse_tags(element: &ElementRef) -> Option<Vec<String>> {
    let selector = Selector::parse("div.more a:not(.topic)").unwrap();
    let element = element.select(&selector);

    Some(
        element
            .map(|tag| tag.text().next().unwrap().into())
            .collect::<Vec<String>>(),
    )
}

#[cfg(test)]
mod tests {
    use crate::domain::{Article, NewsSource};
    use crate::services::dou::article_scraper::parse_articles;
    use scraper::Html;

    #[test]
    fn parse_article_correctly() {
        let document = r#"<body><div class="b-lenta"><article><h2><a href="article_link">Article title</a></h2><p>Article description</p><div class="more"><a class="topic">Topic</a><a>Tag1</a><a>Tag2</a></div></article></div></body>"#;
        let expected = Article::new(
            "Article title".into(),
            Some("Article description".into()),
            "article_link".into(),
            NewsSource::Dou,
            Some(vec!["Tag1".into(), "Tag2".into()]),
        );

        let actual = parse_articles(&Html::parse_document(document)).unwrap();

        assert_eq!(actual, vec![expected])
    }
}
