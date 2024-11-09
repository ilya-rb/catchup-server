use anyhow::{bail, Result};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::domain::{Article, NewsSource, NewsSourceKind::IrishTimes, Tag, Tags};

#[tracing::instrument("Scrape irish times articles")]
pub async fn scrape_latest_articles(
    http_client: &Client,
    url: &Url,
    tag: Tag,
) -> Result<Vec<Article>> {
    let response = http_client
        .get(url.as_ref())
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(url, &document, tag)?;

    Ok(articles)
}

struct Headline {
    pub text: String,
    pub href: String,
}

fn parse_articles(url: &Url, document: &Html, tag: Tag) -> Result<Vec<Article>> {
    let selector = match Selector::parse("article") {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to find articles by selector {:?}", e);
            return Ok(vec![]);
        }
    };

    let articles = document
        .select(&selector)
        .filter_map(|article| {
            let headline = match parse_headline(&article) {
                Ok(h) => h,
                Err(e) => {
                    tracing::error!("Failed to parse article headline, skipping {:?}", e);
                    return None;
                }
            };

            let description = match parse_description(&article) {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("Failed to parse article description, skipping {:?}", e);
                    return None;
                }
            };

            let mut url = url.clone();
            url.set_path(headline.href.as_str());

            Article::new(
                headline.text,
                description,
                url,
                NewsSource::of_kind(IrishTimes),
                Tags(vec![tag.clone()]),
            )
            .map_err(|e| tracing::error!("Failed to create article, skipping {:?}", e))
            .ok()
        })
        .collect::<Vec<Article>>();

    Ok(articles)
}

fn parse_headline(article: &ElementRef) -> Result<Headline> {
    let selector = Selector::parse("h2 a").expect("Failed to parse selector");
    let element = article.select(&selector).next().unwrap();

    let text = match element.text().next() {
        None => bail!("Text is missing from headline"),
        Some(t) => t,
    };

    let href = match element.value().attr("href") {
        None => bail!("Headline href is missing"),
        Some(h) => h,
    };

    Ok(Headline {
        text: String::from(text),
        href: String::from(href),
    })
}

//noinspection DuplicatedCode
fn parse_description(article: &ElementRef) -> Result<Option<String>> {
    let selector = Selector::parse("p a").expect("Failed to parse selector");

    match article.select(&selector).next() {
        None => {
            tracing::warn!("Article does not contain description element");
            Ok(None)
        }
        Some(element) => Ok(element.text().next().map(String::from)),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use scraper::Html;
    use url::Url;

    use crate::domain::{Article, NewsSource, NewsSourceKind::IrishTimes, Tag, Tags};

    use super::parse_articles;

    #[rstest]
    #[case(
        r#"<body><div><article><div><h2><a href="/path/to/article">Title</a></h2><p><a>Description</a></p></div></article></div></body>"#,
        Article::new(
            String::from("Title"),
            Some(String::from("Description")),
            Url::parse("https://irishtimes.com/path/to/article").unwrap(),
            NewsSource::of_kind(IrishTimes),
            Tags(vec![Tag::new(String::from("Technology")).unwrap()]),
        ).unwrap()
    )]
    #[case(
        r#"<body><div><article><div><h2><a href="path/to/article">Title</a></h2></div></article></div></body>"#,
        Article::new(
            String::from("Title"),
            None,
            Url::parse("https://irishtimes.com/path/to/article").unwrap(),
            NewsSource::of_kind(IrishTimes),
            Tags(vec![Tag::new(String::from("Technology")).unwrap()]),
        ).unwrap()
    )]
    fn parse_article_correctly(#[case] html: String, #[case] expected: Article) {
        let tag = Tag::new(String::from("Technology")).unwrap();
        let url = Url::parse("https://irishtimes.com").unwrap();
        let actual = parse_articles(&url, &Html::parse_fragment(&html), tag).unwrap();

        assert_eq!(actual, vec![expected]);
    }
}
