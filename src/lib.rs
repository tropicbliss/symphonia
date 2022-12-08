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
  pub blocking: Option<bool>,
}

#[napi]
pub fn play(buf: Buffer, opt: Options) -> Result<()> {
  let buf: Vec<u8> = buf.into();
  let volume = opt.volume.unwrap_or(1.0) as f32;
  let blocking = opt.blocking.unwrap_or(true);
  let (_stream, stream_handle) =
    OutputStream::try_default().map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  let sink =
    Sink::try_new(&stream_handle).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  sink.set_volume(volume);
  let cursor = Cursor::new(buf);
  let source =
    Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  sink.append(source);
  if blocking {
    sink.sleep_until_end();
  }
  Ok(())
}
