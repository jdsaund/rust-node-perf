const rustAdddon = require('../rust-addon')
const wasmPack = require('../wasm-pack/pkg')

console.log(rustAdddon.add(2, 3))
console.log(wasmPack.add(2, 3))
