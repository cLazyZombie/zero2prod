use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

#[actix_rt::test]
async fn health_check_works() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect Postgres.");

    let server = zero2prod::run(listener, connection).expect("Fail to run server");

    let _h = actix_rt::spawn(async move {
        let _ = server.await;
    });

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_from_data() {
    let addr = spawn_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect Postgres.");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let addr = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www.form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not filed with 400 BAD Request when the payload was {}.",
            error_message
        );
    }
}
