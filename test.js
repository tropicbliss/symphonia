const { Worker, isMainThread, parentPort } = require("worker_threads");

if (isMainThread) {
    const worker = new Worker(__filename);
    worker.on("message", (msg) => console.log(msg))
    console.log("hi")
} else {
    console.log("hi")
    sleep(10000).then((e) => {
        parentPort.postMessage("Hello world!")
    })
}

function sleep(ms) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
}