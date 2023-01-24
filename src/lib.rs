#![deny(clippy::all)]

use crossbeam_channel::{bounded, select, tick, Receiver, Sender};
use napi::{
  bindgen_prelude::{Buffer, ClassInstance, Error, Status},
  Env, Result,
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

enum ControllerMessages {
  Play,
  Pause,
  Stop,
  SetSpeed(f32),
  SetVolume(f32),
}

#[napi]
/// Provides methods to play, pause, and stop the audio.
pub struct Controller {
  tx: Sender<ControllerMessages>,
}

#[napi]
impl Controller {
  #[napi]
  /// Resumes playback.
  pub fn play(&self) -> Result<()> {
    self
      .tx
      .send(ControllerMessages::Play)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  /// Pauses playback. No effect if already paused. A paused controller can be resumed with `play()`.
  pub fn pause(&self) -> Result<()> {
    self
      .tx
      .send(ControllerMessages::Pause)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  /// Stops the playback. Once stopped, the audio track can never be played again.
  pub fn stop(&self) -> Result<()> {
    self
      .tx
      .send(ControllerMessages::Stop)
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  /// Sets the playback speed.
  pub fn set_speed(&self, speed: f64) -> Result<()> {
    self
      .tx
      .send(ControllerMessages::SetSpeed(speed as f32))
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }

  #[napi]
  /// Sets the playback volume.
  pub fn set_volume(&self, volume: f64) -> Result<()> {
    self
      .tx
      .send(ControllerMessages::SetVolume(volume as f32))
      .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
  }
}

#[napi(object)]
pub struct Data {
  pub channels: u16,
  pub current_frame_len: Option<u32>,
  pub sample_rate: u32,
  /// In seconds.
  pub total_duration: Option<u32>,
  /// Provides controls to play, pause, and stop the audio.
  pub controller: Option<ClassInstance<Controller>>,
}

#[napi]
/// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
/// The speed and volume is both set to 1.0 by default. Take note that a controller is only returned if the method is non-blocking,
pub fn play_from_buf(buf: Buffer, opt: Option<Options>, env: Env) -> Result<Data> {
  let buffer: Vec<u8> = buf.into();
  let cursor = Cursor::new(buffer);
  let source =
    Decoder::new(cursor).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  handler(source, opt, env)
}

#[napi]
/// This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead.
/// The speed and volume is both set to 1.0 by default. Take note that a controller is only returned if the method is non-blocking,
pub fn play_from_sine(freq: f64, ms: u32, opt: Option<Options>, env: Env) -> Result<Data> {
  let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(ms as u64));
  handler(source, opt, env)
}

#[inline]
fn handler<S>(source: S, opt: Option<Options>, env: Env) -> Result<Data>
where
  S: Source + Send + 'static,
  S::Item: Sample + Send,
{
  let is_blocking = if let Some(opt) = &opt {
    opt.is_blocking.unwrap_or(true)
  } else {
    true
  };
  if is_blocking {
    let data = Data {
      channels: source.channels(),
      current_frame_len: source
        .current_frame_len()
        .map(|i| i.try_into().unwrap_or(u32::MAX)),
      sample_rate: source.sample_rate(),
      total_duration: source
        .total_duration()
        .map(|d| d.as_secs().try_into().unwrap_or(u32::MAX)),
      controller: None,
    };
    play_blocking(source, opt)?;
    Ok(data)
  } else {
    let (tx, rx) = bounded::<ControllerMessages>(1);
    let controller = Some(Controller { tx }.into_instance(env)?);
    let data = Data {
      channels: source.channels(),
      current_frame_len: source
        .current_frame_len()
        .map(|i| i.try_into().unwrap_or(u32::MAX)),
      sample_rate: source.sample_rate(),
      total_duration: source
        .total_duration()
        .map(|d| d.as_secs().try_into().unwrap_or(u32::MAX)),
      controller,
    };
    std::thread::spawn(|| {
      play(source, opt, rx).unwrap();
    });
    Ok(data)
  }
}

#[inline]
fn play_blocking<S>(source: S, opt: Option<Options>) -> Result<()>
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

#[inline]
fn play<S>(source: S, opt: Option<Options>, rx: Receiver<ControllerMessages>) -> Result<()>
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
  let ticker = tick(Duration::from_secs(3));
  loop {
    select! {
      recv(rx) -> rx => {
        match rx {
          Ok(ControllerMessages::Play) => sink.play(),
          Ok(ControllerMessages::Pause) => sink.pause(),
          Ok(ControllerMessages::Stop) => {
            sink.stop();
            // Once the sink is stopped, the queue is cleared, and no more audio can be played, so we might as well stop the thread right now.
            break;
          },
          Ok(ControllerMessages::SetSpeed(speed)) => sink.set_speed(speed),
          Ok(ControllerMessages::SetVolume(volume)) => sink.set_volume(volume),
          Err(_) => {
            // When the controller gets garbage collected and tx drops, we don't want the playing to stop
            if !sink.is_paused() {
              sink.sleep_until_end();
            }
            break;
          }
        }
      },
      recv(ticker) -> _ => {
        // Makes sure this thread ends instead of forever looping until Node.js stops
        if sink.empty() {
            break;
        }
      }
    }
  }
  Ok(())
}
