use audio_streamer::configuration::get_configuration;
use audio_streamer::startup::run;
use audio_streamer::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("audio_streamer".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
            &configuration.database.connection_string().expose_secret()
        )
        .expect("Failed to connect to Postgres");

    let address = format!(
        "{}:{}",
         configuration.application.host,
         configuration.application.port,
    );
    let listener = TcpListener::bind(address)?;

    println!("Server started!");
    run(listener, connection_pool)?.await
}
