# rust-node-perf  

Benchmarking the execution overheads of various frameworks used in interfacing rust with node.js. Many of the competing solutions arent clear on what approach is being used (FFI, N-API, WASM), which is a primary factor in the per-call performance overhead.

This project provides a minimal implementation of a function `add(a, b)` in each framework to benchmark these overheads so we can compare their performance.

## Results

| Framework                     | Relative Exec Time | Effort   |
| ----------------------------- | ------------------ | -------- |
| node.js                       | 1                  | n/a      |
| wasm-pack (nodejs target)     | 1.5386953312994696 | low      |
| rust-addon                    | 2.563630295032209  | high     |
| napi-rs                       | 3.1991337066589773 | mid      |
| neon                          | 13.342197321199631 | mid      |
| node-bindgen                  | 13.606728128895583 | low      |
