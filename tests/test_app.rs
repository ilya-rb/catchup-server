use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use catchup_server::telemetry::LogLevel;
use catchup_server::{app, configuration, telemetry};

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        telemetry::init_tracing("test".into(), LogLevel::Info, std::io::stdout);
    } else {
        telemetry::init_tracing("test".into(), LogLevel::Info, std::io::sink);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = configuration::read_configuration().expect("Failed to read config");
        c.database.database_name = Uuid::new_v4().to_string();
        c.app.port = 0;
        c
    };

    configure_database(&configuration.database).await;

    let server = app::App::build(configuration.clone())
        .await
        .expect("Failed to build server");

    let port = server.port();
    let _ = tokio::spawn(server.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
        db_pool: PgPoolOptions::new().connect_lazy_with(configuration.database.with_db()),
    }
}

pub async fn configure_database(config: &configuration::DatabaseSettings) {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to establish connection pool");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
}
