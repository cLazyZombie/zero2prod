pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, subscribe};
use sqlx::PgConnection;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    let connection = actix_web::web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("subscriptions", web::post().to(subscribe))
            .data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
