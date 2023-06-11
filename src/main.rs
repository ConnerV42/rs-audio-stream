use std::convert::Infallible;
use warp::{Filter, http::Response, hyper::Body};
use bytes::Bytes;

async fn stream_audio(format: String, name: Option<String>) -> Result<Box<dyn warp::Reply>, Infallible> {
    if let Some(name) = name {
        let file_path = format!("audio/{}.{}", name, format);
        if std::path::Path::new(&file_path).exists() {
            let data = std::fs::read(file_path).unwrap();
            let content_type = match format.as_str() {
                "wav" => "audio/wav",
                "mp3" => "audio/mpeg",
                _ => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST)),
            };
            let stream = futures::stream::iter(data.into_iter().map(|byte| Ok::<_, Infallible>(Bytes::from(vec![byte]))));
            let response = Response::builder()
                .header("Content-Type", content_type)
                .body(Body::wrap_stream(stream))
                .unwrap();
            return Ok(Box::new(response));
        }
    }

    Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .build();

    let audio_route = warp::path("audio")
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::param().map(Some).or_else(|_| async { Ok::<_, Infallible>((None,)) }))
        .and_then(stream_audio)
        .with(cors);

    println!("Server started at http://localhost:8080");
    warp::serve(audio_route).run(([127, 0, 0, 1], 8080)).await;
}

