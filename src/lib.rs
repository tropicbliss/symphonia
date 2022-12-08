#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{Buffer, Error, Status},
  Result,
};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Options {
  pub volume: Option<f64>,
}

impl Default for Options {
  fn default() -> Self {
    Self { volume: Some(1.0) }
  }
}

#[napi]
pub fn play(buf: Buffer, opt: Option<Options>) -> Result<()> {
  let opt = opt.unwrap_or_default();
  let buf: Vec<u8> = buf.into();
  let volume = opt.volume.unwrap_or(1.0) as f32;
  let (_stream, stream_handle) =
    OutputStream::try_default().map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  let sink =
    Sink::try_new(&stream_handle).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  sink.set_volume(volume);
  let cursor = Cursor::new(buf);
  let source =
    Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  sink.append(source);
  sink.sleep_until_end();
  Ok(())
}
