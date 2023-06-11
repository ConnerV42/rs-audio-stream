use std::convert::Infallible;
use std::io::Cursor;

use hound::{WavSpec, WavWriter};
use warp::{Filter, http::Response, hyper::Body};
use bytes::Bytes;
use lame::Lame; // https://en.wikipedia.org/wiki/LAME

/// LAME recommends a buffer size of 7200 bytes for MP3 encoding because it is large enough
///
/// to accommodate the maximum possible size of an encoded MP3 frame.
/// In practice, most MP3 frames will be smaller than 7200 bytes, but using this recommended buffer size ensures
/// that the buffer will always be large enough to hold any MP3 frame produced by the LAME encoder.
const MP3_BUFFER_SIZE: usize = 7200;

/// Number of samples per MP3 frame for the specific encoding settings.
///
/// This constant represents the number of samples per MP3 frame used in the encoding
/// process. It is based on the MPEG-1 Layer III format with a sample rate of 44.1 kHz
/// and a single channel (mono).
const SAMPLES_PER_MP3_FRAME: usize = 1152;

/// Sample rate for the audio data in samples per second.
///
/// This constant specifies the number of samples per second in the audio data.
/// A sample rate of 44,100 Hz (44.1 kHz) is a common value used in digital audio
/// and provides good audio quality.
const SAMPLE_RATE: u32 = 44100;

/// Frequency of the sine wave tone in Hertz (Hz).
///
/// This constant specifies the frequency of the sine wave tone generated in the
/// audio data. A frequency of 440 Hz corresponds to the musical note A4, which
/// is often used as a tuning reference for musical instruments.
const TONE_FREQ: f32 = 440.0;

/// Duration of the generated audio in seconds.
///
/// This constant specifies the duration of the generated audio data. The value
/// of 5 seconds has been chosen to provide a reasonably long audio sample for
/// demonstration purposes.
const DURATION_SECS: u32 = 10;

/// Bits per sample for the audio data.
///
/// This constant specifies the number of bits used to represent each audio sample.
/// A value of 16 bits per sample is a common choice for digital audio, as it provides
/// a good balance between audio quality and data size. With 16 bits per sample, the
/// dynamic range (the difference between the quietest and loudest sounds) is about
/// 96 decibels (dB), which is suitable for most applications.
const BITS_PER_SAMPLE: u16 = 16;

fn create_sine_wave_wav() -> Vec<u8> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let mut buffer = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut buffer, spec).unwrap();
        let amplitude = i16::MAX as f32 / 3.0; // Divide by the number of frequencies to avoid clipping

        let frequencies = [TONE_FREQ, 554.37, 659.25];

        for t in 0..DURATION_SECS * SAMPLE_RATE {
            let t = t as f32 / SAMPLE_RATE as f32;
            let mut sample = 0.0;

            for &freq in frequencies.iter() {
                sample += (freq * t * 2.0 * std::f32::consts::PI).sin() * amplitude;
            }

            writer.write_sample(sample as i16).unwrap();
        }
    }

    buffer.into_inner()
}

fn create_sine_wave_mp3() -> Vec<u8> {
    let wav_data = create_sine_wave_wav();
    let wav_reader = Cursor::new(wav_data);
    let mut wav_reader = hound::WavReader::new(wav_reader).unwrap();

    let mut mp3_data = Vec::new();
    let mut lame_encoder = Lame::new().unwrap();

    lame_encoder.set_sample_rate(SAMPLE_RATE).unwrap();
    lame_encoder.set_channels(1).unwrap();
    lame_encoder.init_params().unwrap();

    let mut samples_buffer = Vec::new();
    for result in wav_reader.samples::<i16>() {
        let sample = result.unwrap();
        samples_buffer.push(sample);
        if samples_buffer.len() >= SAMPLES_PER_MP3_FRAME {
            let mut mp3_buffer = vec![0u8; MP3_BUFFER_SIZE];
            let bytes_written = lame_encoder.encode(&samples_buffer, &samples_buffer, &mut mp3_buffer).unwrap();
            mp3_data.extend_from_slice(&mp3_buffer[..bytes_written]);
            samples_buffer.clear();
        }
    }

    if !samples_buffer.is_empty() {
        let mut mp3_buffer = vec![0u8; MP3_BUFFER_SIZE];
        let bytes_written = lame_encoder.encode(&samples_buffer, &samples_buffer, &mut mp3_buffer).unwrap();
        mp3_data.extend_from_slice(&mp3_buffer[..bytes_written]);
    }

    let mut mp3_buffer = vec![0u8; MP3_BUFFER_SIZE];
    let bytes_written = lame_encoder.encode(&[], &[], &mut mp3_buffer).unwrap();
    mp3_data.extend_from_slice(&mp3_buffer[..bytes_written]);

    mp3_data
}

async fn stream_audio(format: String) -> Result<Box<dyn warp::Reply>, Infallible> {
    let data = match format.as_str() {
        "wav" => create_sine_wave_wav(),
        "mp3" => create_sine_wave_mp3(),
        _ => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST)),
    };

    let content_type = match format.as_str() {
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        _ => unreachable!(),
    };

    let stream = futures::stream::iter(data.into_iter().map(|byte| Ok::<_, Infallible>(Bytes::from(vec![byte]))));

    let response = Response::builder()
        .header("Content-Type", content_type)
        .body(Body::wrap_stream(stream))
        .unwrap();

    Ok(Box::new(response))
}

#[tokio::main] // https://tokio.rs/
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Hx-Current-Url",
            "Hx-Request",
            "Referer",
            "Sec-Ch-Ua",
            "Sec-Ch-Ua-Mobile",
            "Sec-Ch-Ua-Platform",
            "Content-Type",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
            "Access-Control-Allow-Credentials",
        ])
        .allow_methods(vec!["GET", "OPTIONS"]).build();

    let audio_route = warp::path("audio")
        .and(warp::get())
        .and(warp::path::param())
        .and_then(stream_audio)
        .with(cors);

    println!("Server started at http://localhost:8080");
    warp::serve(audio_route).run(([127, 0, 0, 1], 8080)).await;
}
