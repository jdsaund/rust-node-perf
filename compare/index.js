const rustAdddon = require('../rust-addon')
const wasmPack = require('../wasm-pack/pkg')
const neon = require('../neon')

console.log(rustAdddon.add(2, 3))
console.log(wasmPack.add(2, 3))
console.log(neon.add(2, 3))
