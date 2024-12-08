use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::domain::{Article, NewsSource, NewsSourceKind::IrishTimes, Tag, Tags};

pub async fn scrape_latest_articles(http_client: &Client, base_url: &Url) -> Result<Vec<Article>> {
    let today: DateTime<Utc> = Utc::now();
    let today = today.format("%Y/%m/%d").to_string();
    let url = Url::parse(format!("{}/{}", base_url, today).as_str())?;
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);
    let articles = parse_articles(base_url, &document)?;

    Ok(articles)
}

struct Headline {
    pub text: String,
    pub href: String,
}

fn parse_articles(url: &Url, document: &Html) -> Result<Vec<Article>> {
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

            let tag = match parse_tag(&article) {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Failed to parse article tag, skipping {:?}", e);
                    return None;
                }
            };

            let mut url = url.clone();
            url.set_path(headline.href.as_str());

            Article::new(
                headline.text,
                None,
                url,
                NewsSource::of_kind(IrishTimes),
                Tags(vec![tag.clone()]),
                None,
                None,
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

fn parse_tag(article: &ElementRef) -> Result<Tag> {
    let selector = Selector::parse("div.c-grid a").expect("Failed to parse tag selector");
    let element = article.select(&selector).next().unwrap();

    let text = match element.text().next() {
        None => bail!("Text is missing from tag selector"),
        Some(t) => t,
    };

    Tag::new(String::from(text))
}

#[cfg(test)]
mod tests {
    use scraper::Html;
    use url::Url;

    use crate::domain::{Article, NewsSource, NewsSourceKind::IrishTimes, Tag, Tags};

    use super::parse_articles;

    const SAMPLE_HTML: &str = r#"
    <body>
        <div>
            <article>
                <div class="c-grid">
                    <div>
                        <span><a>Article tag</a></span>
                    </div>
                </div>
                <div>
                    <h2><a href="path-to-article">Article title</a></h2>
                </div>
            </article>
        </div>
    </body>
    "#;

    #[test]
    fn parse_article_correctly() {
        let url = Url::parse("https://irishtimes.com").unwrap();

        let expected = vec![Article::new(
            String::from("Article title"),
            None,
            Url::parse("https://irishtimes.com/path-to-article").unwrap(),
            NewsSource::of_kind(IrishTimes),
            Tags(vec![Tag::new(String::from("Article tag")).unwrap()]),
            None,
            None,
        )
        .unwrap()];
        let actual = parse_articles(&url, &Html::parse_fragment(SAMPLE_HTML)).unwrap();

        assert_eq!(actual, expected);
    }
}
