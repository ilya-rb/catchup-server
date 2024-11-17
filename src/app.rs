use actix_web::web::Data;
use actix_web::{web, HttpServer};
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::api;
use crate::configuration::Settings;

pub struct App {
    pub db_pool: PgPool,
    pub http_client: Client,
    pub port: u16,
    pub request_listener: TcpListener,
    pub settings: Settings,
}

impl App {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        Self::build_internal(settings, None).await
    }

    pub async fn with_custom_db(
        settings: Settings,
        db_pool: PgPool,
    ) -> Result<Self, std::io::Error> {
        Self::build_internal(settings, Some(db_pool)).await
    }

    async fn build_internal(
        settings: Settings,
        db_pool: Option<PgPool>,
    ) -> Result<Self, std::io::Error> {
        let db_pool = db_pool.unwrap_or_else(|| {
            PgPoolOptions::new().connect_lazy_with(settings.database.connect_options())
        });

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
            settings,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        let db = Data::new(self.db_pool);
        let http_client = Data::new(self.http_client);
        let settings = Data::new(self.settings);

        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .app_data(db.clone())
                .app_data(http_client.clone())
                .app_data(settings.clone())
                .route("/healthcheck", web::get().to(api::health_check))
                .route("/news", web::get().to(api::get_news))
                .route("/supported_sources", web::get().to(api::supported_sources))
                .service(actix_files::Files::new("/assets", "./static/"))
        })
        .listen(self.request_listener)?
        .run();

        server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
