use crate::test_app::spawn_app;

#[tokio::test]
pub async fn health_check_returns_200() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/healthcheck", &app.address))
        .send()
        .await;

    claims::assert_ok!(response);
}
