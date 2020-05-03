const rustAdddon = require('./rust-addon')
const wasmPack = require('./wasm-pack/pkg')
const neon = require('./neon')
const napiRs = require('./napi-rs-addon')
const nodeBindgen = require('./node-bindgen-addon')

console.log(rustAdddon.add(2, 3))
console.log(wasmPack.add(2, 3))
console.log(neon.add(2, 3))
console.log(napiRs.add(2, 3))
console.log(nodeBindgen.add(2, 3))