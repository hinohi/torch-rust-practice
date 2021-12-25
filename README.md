# torch-rust-practice

## やりたいこと

PyTorch(tch-rc) → ONNX → Some ONNX runtime → wasm → Predict in browse

## 今

tch で学習はできる。

```rust
VarStore::save("path")
```

で保存もできる。

tch-rc には ONNX に変換する機能がなさそうなので PyTorch (CPython) を経由してみる。

```python
import torch
model = torch.jit.load('vs.zip')
model.eval()
dummy_input = torch.randn(10, 1, 32, 32, device='cpu')
torch.onnx.export(model, dummy_input, 'nmist.onnx')
```

しかしエラー

```
/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py:356: UserWarning: Model has no forward function
  warnings.warn("Model has no forward function")
Traceback (most recent call last):
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py", line 414, in _create_jit_graph
    graph = model.forward.graph
  File "/path/to/torch/lib/python3.9/site-packages/torch/jit/_script.py", line 757, in __getattr__
    return super(RecursiveScriptModule, self).__getattr__(attr)
  File "/path/to/torch/lib/python3.9/site-packages/torch/jit/_script.py", line 474, in __getattr__
    return super(ScriptModule, self).__getattr__(attr)
  File "/path/to/torch/lib/python3.9/site-packages/torch/nn/modules/module.py", line 1177, in __getattr__
    raise AttributeError("'{}' object has no attribute '{}'".format(
AttributeError: 'RecursiveScriptModule' object has no attribute 'forward'

The above exception was the direct cause of the following exception:

Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/__init__.py", line 316, in export
    return utils.export(model, args, f, export_params, verbose, training,
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py", line 107, in export
    _export(model, args, f, export_params, verbose, training, input_names, output_names,
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py", line 724, in _export
    _model_to_graph(model, args, verbose, input_names,
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py", line 493, in _model_to_graph
    graph, params, torch_out, module = _create_jit_graph(model, args)
  File "/path/to/torch/lib/python3.9/site-packages/torch/onnx/utils.py", line 425, in _create_jit_graph
    raise RuntimeError("'forward' method must be a script method") from e
RuntimeError: 'forward' method must be a script method
```

`model.state_dict()` とか `model._c.dump_to_str()` とかでモデル構造は取れるからそこから何かする？
