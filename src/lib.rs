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
#[derive(Clone)]
pub struct Options {
  pub volume: Option<f64>,
  pub speed: Option<f64>,
}

#[napi(object)]
pub struct Data {
  pub channels: u16,
  pub current_frame_len: Option<u32>,
  pub sample_rate: u32,
  /// In seconds
  pub total_duration: Option<u32>,
}

#[napi(js_name = "Audio")]
pub struct JsAudio {
  sink: Sink,
  opt: Option<Options>,
}

#[napi]
impl JsAudio {
  #[napi(constructor)]
  /// The speed and volume is both set to 1.0 by default.
  pub fn with_volume(opt: Option<Options>) -> Result<Self> {
    let (_stream, stream_handle) =
      OutputStream::try_default().map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    let sink = Sink::try_new(&stream_handle)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    if let Some(opt) = &opt {
      if let Some(volume) = opt.volume {
        sink.set_volume(volume as f32);
      }
      if let Some(speed) = opt.speed {
        sink.set_speed(speed as f32);
      }
    }
    Ok(Self { sink, opt })
  }

  #[napi]
  /// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
  pub fn play_from_buf(&self, buf: Buffer, is_blocking: Option<bool>) -> Result<Data> {
    let is_blocking = is_blocking.unwrap_or(true);
    let buffer: Vec<u8> = buf.into();
    let cursor = Cursor::new(buffer);
    let source =
      Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    let data = Data {
      channels: source.channels(),
      current_frame_len: source.current_frame_len().map(|i| i as u32),
      sample_rate: source.sample_rate(),
      total_duration: source.total_duration().map(|d| d.as_secs() as u32),
    };
    self.play(source, is_blocking)?;
    Ok(data)
  }

  #[napi]
  /// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
  pub fn play_from_sine(&self, freq: u32, ms: f64, is_blocking: Option<bool>) -> Result<Data> {
    let is_blocking = is_blocking.unwrap_or(true);
    let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(ms as u64));
    let data = Data {
      channels: source.channels(),
      current_frame_len: source
        .current_frame_len()
        .map(|i| i.try_into().unwrap_or(u32::MAX)),
      sample_rate: source.sample_rate(),
      total_duration: source
        .total_duration()
        .map(|d| d.as_millis().try_into().unwrap_or(u32::MAX)),
    };
    self.play(source, is_blocking)?;
    Ok(data)
  }

  fn play<S>(&self, source: S, is_blocking: bool) -> Result<()>
  where
    S: Source + Send + 'static,
    S::Item: Sample + Send,
  {
    if is_blocking {
      self.sink.append(source);
      self.sink.sleep_until_end();
    } else {
      let opt = self.opt.clone();
      std::thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        if let Some(opt) = &opt {
          if let Some(volume) = opt.volume {
            sink.set_volume(volume as f32);
          }
          if let Some(speed) = opt.speed {
            sink.set_speed(speed as f32);
          }
        }
        sink.append(source);
        sink.sleep_until_end();
      });
    }
    Ok(())
  }
}
