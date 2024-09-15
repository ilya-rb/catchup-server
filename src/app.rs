use std::net::TcpListener;

use actix_web::{web, HttpServer};
use actix_web::web::Data;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::api;
use crate::configuration::Settings;

pub struct App {
    db_pool: PgPool,
    http_client: Client,
    port: u16,
    request_listener: TcpListener,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        let db_pool = PgPoolOptions::new().connect_lazy_with(settings.database.with_db());
        let address = format!("{}:{}", settings.app.host, settings.app.port);
        let request_listener = TcpListener::bind(address)?;
        let port = request_listener.local_addr()?.port();
        let http_client = Client::builder()
            .timeout(settings.http_client.timeout())
            .build()
            .unwrap();

        Ok(Self {
            request_listener,
            db_pool,
            http_client,
            port,
        })
    }

    pub async fn run_until_stopped(self, settings: Settings) -> Result<(), std::io::Error> {
        let db_pool = web::Data::new(self.db_pool);
        let http_client = web::Data::new(self.http_client);
        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .route("/healthcheck", web::get().to(api::health_check))
                .route("/news", web::get().to(api::get_news))
                .route("/scraper", web::get().to(api::run_scraper))
                .route("/supported_sources", web::get().to(api::supported_sources))
                .app_data(db_pool.clone())
                .app_data(http_client.clone())
                .app_data(Data::new(settings.clone()))
                .service(
                    actix_files::Files::new("/images", "./static/")
                        .show_files_listing()
                        .use_last_modified(true),
                )
        })
        .listen(self.request_listener)?
        .run();

        server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
