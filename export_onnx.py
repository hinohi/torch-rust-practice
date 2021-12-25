from collections import OrderedDict
import torch
from torch import nn
from torch.nn.functional import log_softmax, nll_loss


class NN(nn.Module):

    def __init__(self):
        super().__init__()
        model = torch.jit.load('vs.zip').eval()
        self._stack = nn.Sequential(
            nn.Conv2d(1, 32, 5),
            nn.MaxPool2d(2),
            nn.Conv2d(32, 64, 5),
            nn.MaxPool2d(2),
            nn.Flatten(),
            nn.Linear(1024, 1024),
            nn.ReLU(),
            nn.Linear(1024, 10),
            nn.Softmax(-1),
        )
        state_map = [
            ('conv2d_1|', '0.'),
            ('conv2d_2|', '2.'),
            ('linear_1|', '5.'),
            ('linear_2|', '7.'),
        ]
        fixed_state = OrderedDict()
        for key, t in model.state_dict().items():
            for a, b in state_map:
                key = key.replace(a, b)
            fixed_state[key] = t
        self._stack.load_state_dict(fixed_state, strict=True)

    def forward(self, x):
        return self._stack.forward(x)


def main():
    model = NN().cpu()
    print(model)

    dummy_input = torch.randn(1, 1, 28, 28, device='cpu')
    torch.onnx.export(model, dummy_input, 'mnist.onnx')


if __name__ == '__main__':
    main()
