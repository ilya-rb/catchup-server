use crate::configuration::Settings;
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

#[tracing::instrument(name = "Querying supported sources", skip(settings))]
pub async fn supported_sources(settings: web::Data<Settings>) -> HttpResponse {
    let port = settings.app.port;
    let sources = vec![
        as_supported_source(settings.services.irish_times.key.as_str(), port),
        as_supported_source(settings.services.hacker_news.key.as_str(), port),
        as_supported_source(settings.services.dou.key.as_str(), port),
    ];

    let response = web::Json(Response { sources });
    HttpResponse::Ok().json(response)
}

fn as_supported_source(key: &str, port: u16) -> SupportedSource {
    SupportedSource {
        id: String::from(key),
        image_url: format!("http://localhost:{}/images/icons/{}.png", port, key,),
    }
}
