use crate::configuration::Settings;
use actix_web::{web, HttpResponse};
use serde::Serialize;
use url::Url;

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
    let base_url = &settings.app.base_url;
    let port = settings.app.port;
    let sources = vec![
        as_supported_source(base_url, settings.services.irish_times.key.as_str(), port),
        as_supported_source(base_url, settings.services.hacker_news.key.as_str(), port),
        as_supported_source(base_url, settings.services.dou.key.as_str(), port),
    ];

    let response = web::Json(Response { sources });
    HttpResponse::Ok().json(response)
}

fn as_supported_source(base_url: &Url, key: &str, port: u16) -> SupportedSource {
    let mut url = base_url.clone();
    url.set_port(Some(port)).unwrap();
    url.set_path(format!("assets/icons/{}.png", key).as_str());

    SupportedSource {
        id: String::from(key),
        image_url: url.to_string(),
    }
}
