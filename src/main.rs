use std::net::TcpListener;

use audio_streamer::startup::run;
use audio_streamer::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    // const PORT: i32 = 8001;
    let listener = TcpListener::bind(address)?;

    println!("Server started!");
    run(listener)?.await
}

