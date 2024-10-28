use crate::configuration::Settings;
use actix_jobs::Job;
use actix_web::web::Data;
use reqwest::Client;
use sqlx::PgPool;

pub struct ScraperJob {
    pub settings: Data<Settings>,
    pub http_client: Data<Client>,
    pub db_pool: Data<PgPool>,
}

impl Job for ScraperJob {
    fn cron(&self) -> &str {
        self.settings.scraper_config.schedule.as_str()
    }

    #[tracing::instrument(name = "Running scraper job", skip(self))]
    fn run(&mut self) {

    }
}
