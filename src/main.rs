use std::fmt;
use std::error::Error;
use warp::{Filter, reject::Reject};
use warp_range::{filter_range, get_range};

#[derive(Debug)]
struct InvalidFormatError;
impl fmt::Display for InvalidFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid format")
    }
}

impl Error for InvalidFormatError {}

impl Reject for InvalidFormatError {}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .build();

    let audio_route =
        warp::path!("audio" / String / String) // Matches "/audio/<name>/<format>"
        .and(filter_range())
        .and_then(|name: String, format: String, range_header: Option<String>| async move {
            let file_path = format!("audio/{}.{}", name, format);
            match format.as_str() {
                "wav" => get_range(range_header, &file_path, "audio/wav").await,
                "mp3" => get_range(range_header, &file_path, "audio/mpeg").await,
                _ => Err(warp::reject::custom(InvalidFormatError)),
            }
        })
        .with(cors);

    println!("Server started at http://localhost:8080");
    warp::serve(audio_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

