use sqlx::PgPool;
use std::sync::LazyLock;
use uuid::Uuid;

use catchup_server::telemetry::LogLevel;
use catchup_server::{app, configuration, telemetry};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        telemetry::init_tracing("test".into(), LogLevel::Info, std::io::stdout);
    } else {
        telemetry::init_tracing("test".into(), LogLevel::Info, std::io::sink);
    }
});

pub struct TestApp {
    pub app_url: String,
}

impl TestApp {
    pub async fn new(db_pool: PgPool) -> TestApp {
        LazyLock::force(&TRACING);

        let configuration = {
            let mut c = configuration::read_configuration().expect("Failed to read config");
            // Use a different database for each test case
            c.database.database_name = Uuid::new_v4().to_string();
            // Use random OS port
            c.app.port = 0;
            c
        };

        let app = app::App::with_custom_db(configuration.clone(), db_pool)
            .await
            .expect("Failed to build server");

        let port = app.port();

        tokio::spawn(app.run_until_stopped());

        TestApp {
            app_url: format!("http://localhost:{}", port),
        }
    }
}
