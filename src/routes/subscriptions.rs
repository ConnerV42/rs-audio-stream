use actix_web::{web, HttpResponse};
use serde::Deserialize;
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>
) -> HttpResponse {
    let correlation_id = Uuid::new_v4();
    let span = tracing::info_span!(
        "Adding a new subscriber.",
        %correlation_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = span.enter();
    tracing::info!("correlation_id {} - Adding '{}' '{}' as a new subscriber.",
        correlation_id,
        form.email,
        form.name
    );
    tracing::info!(
        "correlation_id {} - Saving new subscriber details in the database",
        correlation_id
    );

    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
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
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "correlation_id {} - New subscriber details have been saved",
                correlation_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "correlation_id {} - Failed to execute query: {:?}",
                correlation_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

