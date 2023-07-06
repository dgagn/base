use crate::spawn_app;

#[tokio::test]
async fn health_check_returns_200() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health-check", addr))
        .send()
        .await
        .expect("Failed to execute request. ");

    assert_eq!(response.status().as_u16(), 200);
}
