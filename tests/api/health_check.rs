use crate::test_app::TestApp;
use sqlx::PgPool;

#[sqlx::test]
pub async fn health_check_returns_200(db_pool: PgPool) {
    let app = TestApp::new(db_pool).await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/healthcheck", &app.address.0))
        .send()
        .await;

    assert!(response.unwrap().status().is_success());
}
