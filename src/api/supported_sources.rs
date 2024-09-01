use crate::domain::NewsSource;
use actix_web::{web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    sources: Vec<SupportedSource>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedSource {
    pub id: String,
    pub image_url: String,
}

#[tracing::instrument(name = "Querying supported sources")]
pub async fn supported_sources() -> HttpResponse {
    // TODO: Move to config
    let sources = vec![
        SupportedSource{
            id: NewsSource::IrishTimes.key(),
            image_url: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQEey2YHaKAWE4PJ14yz8Z8BmqauVdjMExM5A&s".into()
        },
        SupportedSource {
            id: NewsSource::HackerNews.key(),
            image_url: "https://static-00.iconduck.com/assets.00/hackernews-icon-2048x2048-h8uaqp6j.png".into(),
        },
        SupportedSource {
            id: NewsSource::Dou.key(),
            image_url: "https://images.squarespace-cdn.com/content/v1/57b5b62a725e25b7ca9bd8da/1586188273506-R95E0GGT43UBSS28WM2U/6.png".into(),
        },
    ];

    let response = web::Json(Response { sources });
    HttpResponse::Ok().json(response)
}
