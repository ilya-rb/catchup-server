use actix_jobs::{run_forever, Scheduler};
use actix_web::web::Data;
use actix_web::{web, HttpServer};
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::api;
use crate::configuration::Settings;
use crate::jobs::scraper_job::ScraperJob;

pub struct App {
    pub db_pool: PgPool,
    pub http_client: Client,
    pub port: u16,
    pub request_listener: TcpListener,
    pub settings: Settings,
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
            settings,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        let db_pool = Data::new(self.db_pool);
        let http_client = Data::new(self.http_client);
        let settings = Data::new(self.settings);

        Self::setup_jobs(db_pool.clone(), settings.clone(), http_client.clone());

        let server = HttpServer::new(move || {
            actix_web::App::new()
                .wrap(TracingLogger::default())
                .route("/healthcheck", web::get().to(api::health_check))
                .route("/news", web::get().to(api::get_news))
                .route("/supported_sources", web::get().to(api::supported_sources))
                .app_data(db_pool.clone())
                .app_data(http_client.clone())
                .app_data(settings.clone())
                .service(actix_files::Files::new("/assets", "./static/"))
        })
        .listen(self.request_listener)?
        .run();

        server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    fn setup_jobs(db_pool: Data<PgPool>, settings: Data<Settings>, http_client: Data<Client>) {
        let mut scheduler = Scheduler::new();
        let scraper_job = ScraperJob {
            settings,
            http_client,
            db_pool,
        };

        scheduler.add(Box::new(scraper_job));

        run_forever(scheduler);
    }
}
