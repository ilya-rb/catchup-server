use std::error::Error;

use crate::domain::{Article, NewsSource, NewsSourceKind, Tags};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use url::Url;
use NewsSourceKind::Dou;

#[tracing::instrument("Scraping DOU articles")]
pub async fn scrape_latest_articles(
    http_client: &Client,
    url: Url,
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
            // TODO: Handle error, log and skip broken articles
            let url = Url::parse(link.as_str()).expect("Invalid URL");
            // TODO: Handle error and skip broken articles

            Article::new(
                title,
                Some(description),
                url,
                NewsSource::of_kind(Dou),
                tags,
            )
            .unwrap()
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

fn parse_tags(element: &ElementRef) -> Tags {
    let selector = Selector::parse("div.more a:not(.topic)").unwrap();
    let element = element.select(&selector);

    element
        .map(|tag| tag.text().next().unwrap().into())
        .filter(|t: &String| !t.is_empty())
        .collect::<Vec<String>>()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::domain::{Article, NewsSource, NewsSourceKind};
    use crate::services::dou::article_scraper::parse_articles;
    use scraper::Html;
    use url::Url;
    use NewsSourceKind::Dou;

    #[test]
    fn parse_article_correctly() {
        let document = r#"<body><div class="b-lenta"><article><h2><a href="https://example.com">Article title</a></h2><p>Article description</p><div class="more"><a class="topic">Topic</a><a>Tag1</a><a>Tag2</a></div></article></div></body>"#;
        let expected = Article::new(
            "Article title".into(),
            Some("Article description".into()),
            Url::parse("https://example.com").unwrap(),
            NewsSource::of_kind(Dou),
            vec![String::from("Tag1"), String::from("Tag2")].into(),
        )
        .unwrap();

        let actual = parse_articles(&Html::parse_document(document)).unwrap();

        assert_eq!(actual, vec![expected])
    }
}
