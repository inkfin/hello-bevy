# Hello BEVY

<https://bevyengine.org/>

<https://github.com/bevyengine/bevy/tree/latest/examples>

```shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "hello-bevy" \
    ./target/wasm32-unknown-unknown/release/hello-bevy.wasm
```

optimize size
```shell
# Optimize for size (z profile).
wasm-opt -Oz -o output.wasm input.wasm

# Optimize for size (s profile).
wasm-opt -Os -o output.wasm input.wasm

# Optimize for speed.
wasm-opt -O3 -o output.wasm input.wasm

# Optimize for both size and speed.
wasm-opt -O -ol 100 -s 100 -o output.wasm input.wasm
```