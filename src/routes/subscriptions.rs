use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use sqlx::PgConnection;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct SubscribeFormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    _form: Form<SubscribeFormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
