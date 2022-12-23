/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Options {
  volume?: number
  speed?: number
}
export interface Data {
  channels: number
  currentFrameLen?: number
  sampleRate: number
  totalDuration?: number
}
export type JsAudio = Audio
export class Audio {
  /** The speed and volume is both set to 1.0 by default. */
  constructor(opt?: Options | undefined | null)
  /** This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead. */
  playFromBuf(buf: Buffer, isBlocking?: boolean | undefined | null): Data
  /** This method blocks the thread by default. Set `isBlocking` to `false` to allow this method to spawn a thread in the background. Note that this incurs some additional overhead. */
  playFromSine(freq: number, ms: number, isBlocking?: boolean | undefined | null): Data
}
