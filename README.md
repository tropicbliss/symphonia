# symphonia.js

A "way too simple" cross-platform zero dependency audio playback library for Node.js

## Supported Platforms

- Windows (x64)
- macOS (x64)
- macOS (ARM64)
- Linux (x64)

## Supported Audio formats

- MP3
- WAV
- Vorbis
- FLAC
- MP4
- AAC

## Note

When I mean zero dependency, I mean zero dependency to a reasonable extent. There still needs to be a system audio library available for Symphonia.js to interface with. For Linux, you'll need ALSA (`libasound2` on Debian/Ubuntu based distros).

## Credits

- [napi-rs](https://github.com/napi-rs/napi-rs)
- [rodio](https://github.com/RustAudio/rodio)

## Usage

```js
const axios = require("axios");
const fs = require("fs");
const symphonia = require("@tropicbliss/symphonia");

try {
    const buf = fs.readFileSync("chime.ogg"); // Gets a Buffer
    symphonia.playFromBuf(buf, { speed: 1.0, volume: 1.0 }) // The second option object is optional. The speed and volume is both set to 1.0 by default.

    // You can also obtain buffers from a web request
    axios.get(URL).then((res) => Buffer.from(res.data, "binary"))
        .then((buf) => {
            symphonia.playFromBuf(buf);
        })
    
    // Play a sine wave at the frequency of 440Hz (about double the frequency of human hearing) for 250ms
    symphonia.playFromSine(440.0, 250);
} catch (e) {
    console.log("Error playing audio: ", e)
}
```

Note that calling `playX()` blocks the main thread so use worker threads to make it non-blocking.

```js
const { Worker, isMainThread, parentPort } = require("worker_threads");
const fs = require("fs");
const symphonia = require("@tropicbliss/symphonia");

if (isMainThread) {
    const worker = new Worker(__filename);
    worker.on("message", (msg) => console.log(msg))
} else {
    const buf = fs.readFileSync("chime.ogg");
    symphonia.playFromBuf(buf);
    parentPort.postMessage("finished playing");
}
```
