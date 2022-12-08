#![deny(clippy::all)]

use napi::{
  bindgen_prelude::{Buffer, Error, Status},
  Result,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::{BufReader, Cursor};

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct Options {
  pub volume: Option<f64>,
}

#[napi]
pub fn play(buf: Buffer, opt: Options) -> Result<()> {
  let buf: Vec<u8> = buf.into();
  let volume = opt.volume.unwrap_or(1.0) as f32;
  play_sound(buf, volume)?;
  Ok(())
}

#[napi]
pub fn play_async(buf: Buffer) {
  let buf: Vec<u8> = buf.into();
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let buf = Cursor::new(buf);
  let file = BufReader::new(buf);
  let source = Decoder::new(file).unwrap();
  stream_handle.play_raw(source.convert_samples()).unwrap();
}

fn play_sound(buf: Vec<u8>, volume: f32) -> Result<()> {
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
