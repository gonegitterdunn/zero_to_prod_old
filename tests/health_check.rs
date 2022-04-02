use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero_to_prod::configuration::get_configuration;
use zero_to_prod::startup::run;

#[tokio::test]
async fn test_health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_subscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Couldn't load configuration file");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Unable to connect to postgres");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Request failed to execute)");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_subscribe_returns_400_for_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (test_case, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "x-www-form-urlencoded")
            .body(test_case)
            .send()
            .await
            .expect("Request failed to execute");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was: {}",
            error_message
        )
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
