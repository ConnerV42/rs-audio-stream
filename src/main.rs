use std::net::TcpListener;

use audio_streamer::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    const PORT: i32 = 8001;
    let listener =
        TcpListener::bind(format!("0.0.0.0:{}", PORT)).expect("Failed to bind to port 8001");

    println!("Server starting on http://0.0.0.0:{}", PORT);
    run(listener)?.await
}
