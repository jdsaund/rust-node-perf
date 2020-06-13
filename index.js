const rustAddon = require('./rust-addon')
const wasmPack = require('./wasm-pack/pkg')
// const neon = require('./neon')
// const napiRs = require('./napi-rs-addon')
// const nodeBindgen = require('./node-bindgen-addon')

const NS_PER_SEC = 1e9
const NUM_ITERATIONS = 1000

const arr = new Array(100000).fill(0).map((v, i) => 1)
const uint8Arr = new Uint32Array(arr)

// const result = rustAddon.sumBuffer()
// console.log(result);

const modules = {
  rustAddon,
  wasmPack,
  // neon,
  // napiRs,
  // nodeBindgen
}

function bench (func, jsTime, bufOrArray) {
  let totalTime = 0
  let result

  for (let i = 0; i < NUM_ITERATIONS; i++) {
    const start = process.hrtime()
    result = func(bufOrArray)
    const diff = process.hrtime(start)
    totalTime += diff[0] + (diff[1] / NS_PER_SEC)
  }

  return { relativeTime: totalTime / jsTime, result }
}

function sumBuffer (buf) {
  let result = 0
  for (const val of buf) {
    result += val
  }
  return result
}

const { relativeTime, result } = bench(sumBuffer, 1, uint8Arr)

const results = [{ module: 'js', relativeTime, result }]

for (const mod of Object.keys(modules)) {
  results.push({ module: mod, ...bench(modules[mod].sumBuffer, relativeTime, uint8Arr) })
}

console.table(results)
