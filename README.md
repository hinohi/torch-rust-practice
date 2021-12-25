# torch-rust-practice

## やりたいこと

PyTorch(tch-rc) → ONNX → Some ONNX runtime → wasm → Predict in browse

## onnx 動いてる

```
$ cargo run --bin=mnist-onnx
   Compiling torch-rust-practice v0.1.0 (/Users/daiju/ghq/github.com/hinohi/torch-rust-practice)
    Finished dev [unoptimized + debuginfo] target(s) in 1.90s
     Running `target/debug/mnist-onnx`
6 81.17%
5 12.41%
0 6.41%
3 0.00%
9 0.00%
8 0.00%
4 0.00%
1 0.00%
7 0.00%
2 0.00%
```
