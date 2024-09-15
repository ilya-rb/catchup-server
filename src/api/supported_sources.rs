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

    let sources: Vec<SupportedSource> = settings
        .supported_sources
        .iter()
        .map(|(_, service)| SupportedSource {
            id: service.key.clone(),
            image_url: build_image(service.key.clone(), settings.app.port.clone()),
        })
        .collect();

    let response = web::Json(Response { sources });
    HttpResponse::Ok().json(response)
}

fn build_image(name: String, port: u16) -> String {
    format!("http://localhost:{}/images/icons/{}.png", port, name)
}
