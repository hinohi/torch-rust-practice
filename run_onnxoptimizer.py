import argparse

import onnx
from onnxoptimizer import optimize

passes = [
    'eliminate_deadend',
    'eliminate_duplicate_initializer',
    'eliminate_identity',
    'eliminate_if_with_const_cond',
    'eliminate_nop_cast',
    'eliminate_nop_dropout',
    'eliminate_nop_flatten',
    'eliminate_nop_monotone_argmax',
    'eliminate_nop_pad',
    'eliminate_nop_transpose',
    'eliminate_unused_initializer',
    'extract_constant_to_initializer',
    'fuse_add_bias_into_conv',
    'fuse_bn_into_conv',
    'fuse_consecutive_concats',
    'fuse_consecutive_log_softmax',
    'fuse_consecutive_reduce_unsqueeze',
    'fuse_consecutive_squeezes',
    'fuse_consecutive_transposes',
    'fuse_matmul_add_bias_into_gemm',
    'fuse_pad_into_conv',
    'fuse_transpose_into_gemm',
    'lift_lexical_references',
]


def main():
    p = argparse.ArgumentParser()
    p.add_argument('src')
    p.add_argument('--output', '-o', required=True)
    args = p.parse_args()
    if args.src == args.output:
        raise ValueError('ちがいそう')

    model = onnx.load(args.src)
    model = optimize(model, passes)
    with open(args.output, 'wb') as f:
        f.write(model.SerializeToString())


if __name__ == '__main__':
    main()
