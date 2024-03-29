# symphonia.js

A "way too simple" cross-platform zero dependency audio playback library for Node.js

## Supported Platforms

- Windows (x86-64)
- Windows (i386) (32-bit)
- macOS (x86-64)
- macOS (arm64)
- Linux (x86-64)

## Supported Audio Formats

- MP3
- WAV
- Vorbis
- FLAC
- MP4
- AAC

## Note

Take note that the `node_modules` generated by npm when installing this package is non-portable across platforms in order to save space. Thus, you'll need to run `npm install` when transferring your project between platforms in order for this package to work correctly (you should not be committing your `node_modules` anyway so for most use cases this shouldn't be a problem).

Lastly, when you call the functions for the first time it might take a few seconds for the computer to respond. This is perfectly normal behaviour as it might be caused by Windows Defender.

## Credits

- [cpal](https://github.com/rustaudio/cpal)
- [napi-rs](https://github.com/napi-rs/napi-rs)
- [rodio](https://github.com/RustAudio/rodio)
- [symphonia](https://github.com/pdeljanov/Symphonia)

## Usage

```js
const axios = require('axios')
const fs = require('fs')
const symphonia = require('@tropicbliss/symphonia')

try {
  const buf = fs.readFileSync('chime.ogg') // Gets a Buffer
  symphonia.playFromBuf(buf, { speed: 1.0, volume: 1.0, isBlocking: true }) // The option object is optional. The speed and volume is both set to 1.0 and `isBlocking` is set to `true` by default.

  // You can also obtain buffers from a web request
  axios
    .get(URL)
    .then((res) => Buffer.from(res.data, 'binary'))
    .then((buf) => {
      symphonia.playFromBuf(buf)
    })

  // Play a sine wave at the frequency of 440Hz for 250ms
  symphonia.playFromSine(440.0, 250)
} catch (e) {
  console.log('Error playing audio: ', e)
}
```

Note that calling `playFromX()` without setting the `isBlocking` option parameter blocks the main thread by default, so pass `false` to `isBlocking` to make the methods non-blocking.

```js
const fs = require('fs')
const symphonia = require('@tropicbliss/symphonia')

function playStuff() {
  const buf = fs.readFileSync('chime.ogg')
  const data = symphonia.playFromBuf(buf, { isBlocking: false })
  console.log("I'm not done yet, do something else to prevent this program from exiting!")
  data.controller.pause()
  data.controller.play()
  data.controller.stop()
}

try {
  playStuff()
} catch (e) {
  console.log('Error playing audio: ', e)
}
```
