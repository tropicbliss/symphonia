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
  pub is_blocking: Option<bool>,
}

#[napi(object)]
pub struct Data {
  pub channels: u16,
  pub current_frame_len: Option<u32>,
  pub sample_rate: u32,
  /// In seconds.
  pub total_duration: Option<u32>,
}

#[napi]
/// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
/// The speed and volume is both set to 1.0 by default.
pub fn play_from_buf(buf: Buffer, opt: Option<Options>) -> Result<Data> {
  let buffer: Vec<u8> = buf.into();
  let cursor = Cursor::new(buffer);
  let source =
    Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  handler(source, opt)
}

#[napi]
/// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
/// The speed and volume is both set to 1.0 by default.
pub fn play_from_sine(freq: u32, ms: f64, opt: Option<Options>) -> Result<Data> {
  let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(ms as u64));
  handler(source, opt)
}

fn handler<S>(source: S, opt: Option<Options>) -> Result<Data>
where
  S: Source + Send + 'static,
  S::Item: Sample + Send,
{
  let data = Data {
    channels: source.channels(),
    current_frame_len: source.current_frame_len().map(|i| i as u32),
    sample_rate: source.sample_rate(),
    total_duration: source.total_duration().map(|d| d.as_secs() as u32),
  };
  let is_blocking = if let Some(opt) = &opt {
    opt.is_blocking.unwrap_or(true)
  } else {
    true
  };
  if is_blocking {
    play(source, opt)?;
  } else {
    std::thread::spawn(|| {
      play(source, opt).unwrap();
    });
  }
  Ok(data)
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
  if let Some(opt) = opt {
    if let Some(volume) = opt.volume {
      sink.set_volume(volume as f32);
    }
    if let Some(speed) = opt.speed {
      sink.set_speed(speed as f32);
    }
  }
  sink.append(source);
  sink.sleep_until_end();
  Ok(())
}
