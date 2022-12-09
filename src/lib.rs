#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{Buffer, Error, Status},
  Result,
};
use rodio::{source::SineWave, Decoder, OutputStream, Sample, Sink, Source};
use std::{io::Cursor, time::Duration};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Options {
  pub volume: Option<f64>,
  pub speed: Option<f64>,
}

#[napi]
/// The speed and volume is both set to 1.0 by default.
/// Note that calling this function blocks the main thread so use worker threads to make it non-blocking.
pub fn play_from_buf(buf: Buffer, opt: Option<Options>) -> Result<()> {
  let buffer: Vec<u8> = buf.into();
  let cursor = Cursor::new(buffer);
  let source =
    Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  play(source, opt)?;
  Ok(())
}

#[napi]
/// Note that calling this function blocks the main thread so use worker threads to make it non-blocking.
pub fn play_from_sine(freq: u32, ms: f64, opt: Option<Options>) -> Result<()> {
  let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(ms as u64));
  play(source, opt)?;
  Ok(())
}

fn play<S>(source: S, opt: Option<Options>) -> Result<()>
where
  S: Source + Send + 'static,
  S::Item: Sample + Send,
{
  let (_stream, stream_handle) =
    OutputStream::try_default().map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  let sink =
    Sink::try_new(&stream_handle).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  if let Some(options) = opt {
    if let Some(volume) = options.volume {
      sink.set_volume(volume as f32);
    }
    if let Some(speed) = options.speed {
      sink.set_speed(speed as f32);
    }
  }
  sink.append(source);
  sink.sleep_until_end();
  Ok(())
}
