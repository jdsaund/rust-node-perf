const rustAddon = require('./rust-addon')
const wasmPack = require('./wasm-pack/pkg')
const neon = require('./neon')
const napiRs = require('./napi-rs-addon')
const nodeBindgen = require('./node-bindgen-addon')

const NS_PER_SEC = 1e9
const NUM_ITERATIONS = 10000

const arr = new Array(25).fill(0).map((v, i) => i)
const buffer = Buffer.from(arr)
const uint8Arr = new Uint8Array(arr)

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

  for (let i = 0; i < NUM_ITERATIONS; i++) {
    const start = process.hrtime()
    func(bufOrArray)
    const diff = process.hrtime(start)
    totalTime += diff[0] + (diff[1] / NS_PER_SEC)
  }

  return totalTime / jsTime
}

function sumBuffer (buf) {
  let result = 0
  for (const val in buf) {
    result += val
  }
  return result
}

const jsTime = bench(sumBuffer, 1)

const results = [{ module: 'js', relativeTime: 1 }]

for (const mod of Object.keys(modules)) {
  const bufOrArray = mod === 'wasmPack' ? uint8Arr : buffer
  results.push({ module: mod, relativeTime: bench(modules[mod].sumBuffer, jsTime, bufOrArray) })
}

console.table(results)
