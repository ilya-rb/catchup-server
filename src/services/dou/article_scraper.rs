use crate::domain::{Article, NewsSource, NewsSourceKind, Tags};
use anyhow::{bail, Result};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use url::Url;
use NewsSourceKind::Dou;

#[tracing::instrument("Scraping DOU articles")]
pub async fn scrape_latest_articles(http_client: &Client, url: Url) -> Result<Vec<Article>> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(&document)?;

    Ok(articles)
}

struct Headline {
    pub text: String,
    pub href: Url,
}

fn parse_articles(document: &Html) -> Result<Vec<Article>> {
    let selector = Selector::parse("div.b-lenta article").expect("Failed to parse selector");
    let articles = document
        .select(&selector)
        .filter_map(|article| {
            let headline = match parse_headline(&article) {
                Ok(h) => h,
                Err(e) => {
                    tracing::error!("Failed to parse headline, skipping: {:?}", e);
                    return None;
                }
            };

            let description = match parse_description(&article) {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("Failed to parse description, skipping: {:?}", e);
                    return None;
                }
            };

            let tags = match parse_tags(&article) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to parse tags, skipping: {:?}", e);
                    return None;
                }
            };

            Article::new(
                headline.text,
                description,
                headline.href,
                NewsSource::of_kind(Dou),
                tags,
            )
            .map_err(|e| tracing::error!("Failed to create article, skipping: {:?}", e))
            .ok()
        })
        .collect::<Vec<Article>>();

    Ok(articles)
}

fn parse_headline(element: &ElementRef) -> Result<Headline> {
    let selector = Selector::parse("h2 a").expect("Failed to parse selector");
    let element = match element.select(&selector).next() {
        Some(e) => e,
        None => bail!("Headline element not found {:?}", element),
    };

    let text = match element.text().next() {
        Some(t) => t,
        None => bail!("Title not found for headline {:?}", element),
    };

    let href = match element.value().attr("href") {
        Some(h) => match Url::parse(h) {
            Ok(h) => h,
            Err(e) => bail!("Error parsing headline href {:?}", e),
        },
        None => bail!("Href not found for headline {:?}", element),
    };

    Ok(Headline {
        text: String::from(text),
        href,
    })
}

//noinspection DuplicatedCode
fn parse_description(element: &ElementRef) -> Result<Option<String>> {
    let selector = Selector::parse("p").expect("Failed to parse selector");

    match element.select(&selector).next() {
        None => {
            tracing::warn!("Article does not contain description element");
            Ok(None)
        }
        Some(e) => Ok(e.text().next().map(String::from)),
    }
}

fn parse_tags(element: &ElementRef) -> Result<Tags> {
    let selector = Selector::parse("div.more a:not(.topic)").expect("Failed to parse selector");
    let element = element.select(&selector);

    Ok(Tags::from(
        element
            .filter_map(|e| e.text().next().map(String::from))
            .collect::<Vec<String>>(),
    ))
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
