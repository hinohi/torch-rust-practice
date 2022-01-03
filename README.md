# torch-rust-practice

成果物: https://hinohi.github.io/torch-rust-practice/

## やりたいこと

PyTorch(tch-rc) → ONNX → Some ONNX runtime → wasm → Predict in browse

## tract-onnx

```
$ cargo run --bin=mnist-onnx --release
   Compiling torch-rust-practice v0.1.0 (/Users/daiju/ghq/github.com/hinohi/torch-rust-practice)
    Finished release [optimized] target(s) in 1.71s
     Running `target/release/mnist-onnx`
load onnx model: 0.023sec
load mnist test data: 0.035sec
run: samples=10000 correct=9930 correct_rate=99.30% 2.624sec
```

`into_optimized` をしなかったパターン。
20倍以上遅い。

```
$ cargo run --bin=mnist-onnx --release
   Compiling torch-rust-practice v0.1.0 (/Users/daiju/ghq/github.com/hinohi/torch-rust-practice)
    Finished release [optimized] target(s) in 1.55s
     Running `target/release/mnist-onnx`
load onnx model: 0.006sec
load mnist test data: 0.025sec
run: samples=10000 correct=9930 correct_rate=99.30% 56.626sec
```

## web

デプロイ

```
wasm-pack build --release web-front
npm run build-prod
npm run deploy
```
