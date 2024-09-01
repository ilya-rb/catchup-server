use actix_web::HttpResponse;

#[tracing::instrument(name = "Running health check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
