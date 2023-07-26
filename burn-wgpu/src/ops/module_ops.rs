use burn_tensor::ops::{
    ConvOptions, ConvTransposeOptions, MaxPool2dBackward, MaxPool2dWithIndices, ModuleOps,
};

use crate::{
    element::{FloatElement, IntElement},
    kernel, GraphicsApi, WgpuBackend,
};

use super::{FloatTensor, IntTensor};

impl<G, F, I> ModuleOps<WgpuBackend<G, F, I>> for WgpuBackend<G, F, I>
where
    G: GraphicsApi + 'static,
    F: FloatElement,
    I: IntElement,
{
    fn conv2d(
        x: FloatTensor<Self, 4>,
        weight: FloatTensor<Self, 4>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvOptions<2>,
    ) -> FloatTensor<Self, 4> {
        kernel::conv::conv2d(x, weight, bias, options)
    }

    fn conv_transpose2d(
        x: FloatTensor<Self, 4>,
        weight: FloatTensor<Self, 4>,
        bias: Option<FloatTensor<Self, 1>>,
        options: ConvTransposeOptions<2>,
    ) -> FloatTensor<Self, 4> {
        kernel::conv::conv_transpose2d(x, weight, bias, options)
    }

    fn avg_pool2d(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> FloatTensor<Self, 4> {
        kernel::pool::avg_pool2d(x, kernel_size, stride, padding)
    }

    fn avg_pool2d_backward(
        x: FloatTensor<Self, 4>,
        grad: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> FloatTensor<Self, 4> {
        kernel::pool::avg_pool2d_backward(x, grad, kernel_size, stride, padding)
    }

    fn max_pool2d(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> FloatTensor<Self, 4> {
        kernel::pool::max_pool2d(x, kernel_size, stride, padding)
    }

    fn max_pool2d_with_indices(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
    ) -> MaxPool2dWithIndices<WgpuBackend<G, F, I>> {
        let (output, indices) =
            kernel::pool::max_pool2d_with_indices(x, kernel_size, stride, padding);

        MaxPool2dWithIndices::new(output, indices)
    }

    fn max_pool2d_with_indices_backward(
        x: FloatTensor<Self, 4>,
        kernel_size: [usize; 2],
        stride: [usize; 2],
        padding: [usize; 2],
        output_grad: FloatTensor<Self, 4>,
        indices: IntTensor<Self, 4>,
    ) -> MaxPool2dBackward<WgpuBackend<G, F, I>> {
        MaxPool2dBackward::new(kernel::pool::max_pool2d_with_indices_backward(
            x,
            output_grad,
            indices,
            kernel_size,
            stride,
            padding,
        ))
    }
}
