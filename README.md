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

When I mean zero dependency, I mean zero dependency to a reasonable extent. There still needs to be a system sound library available for Symphonia.js to interface with. For Linux, you'll need ALSA (`libasound2` on Debian/Ubuntu based distros).

## Credits

- [napi-rs](https://github.com/napi-rs/napi-rs)
- [rodio](https://github.com/RustAudio/rodio)

## Usage

```js
const fs = require("fs");
const symphonia = require("@tropicbliss/symphonia");

const buf = fs.readFileSync("chime.ogg"); // Gets a Buffer
symphonia.play(buf, { speed: 1.0, volume: 1.0 }) // The second option object is optional. The speed and volume is both set to 1.0 by default.
```

You can also obtain buffers from a web request. Note that calling `play()` blocks the main thread so use worker threads to make it concurrent (currently looking for ways to create a non-blocking version of `play()` so you don't have to contend with worker threads).
