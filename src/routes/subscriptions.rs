use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {

    let cid = Uuid::new_v4();

    log::info!(
        "cid {} - Adding '{}' '{}' as a new subscriber.",
        cid,
        form.email,
        form.name
    );

    log::info!("cid {} - Saving new subscriber details in the database", cid);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("cid {} - New subscriber details have been saved", cid);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("cid {} - Failed to execute query: {:?}", cid, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
