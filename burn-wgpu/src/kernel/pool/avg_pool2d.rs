use crate::{
    element::WgpuElement,
    kernel::{
        self, elemwise_workgroup,
        pool::{build_output_and_info_pool2d, build_pool2d_info},
        KernelSettings,
    },
    kernel_wgsl,
    tensor::WgpuTensor,
};

kernel_wgsl!(AvgPool2d, "../../template/pool/avg_pool2d.wgsl");
kernel_wgsl!(
    AvgPool2dBackward,
    "../../template/pool/avg_pool2d_backward.wgsl"
);

pub(crate) fn avg_pool2d<E: WgpuElement>(
    x: WgpuTensor<E, 4>,
    kernel_size: [usize; 2],
    stride: [usize; 2],
    padding: [usize; 2],
) -> WgpuTensor<E, 4> {
    const WORKGROUP: usize = 32;

    let (info_buffer, output) = build_output_and_info_pool2d(&x, kernel_size, stride, padding);
    let kernel = x
        .context
        .compile_static::<KernelSettings<AvgPool2d, E, i32, WORKGROUP, WORKGROUP, 1>>();

    x.context.execute(
        elemwise_workgroup(output.shape.num_elements(), WORKGROUP),
        kernel,
        &[&x.buffer, &output.buffer, &info_buffer],
    );

    output
}

pub(crate) fn avg_pool2d_backward<E: WgpuElement>(
    x: WgpuTensor<E, 4>,
    grad: WgpuTensor<E, 4>,
    kernel_size: [usize; 2],
    stride: [usize; 2],
    padding: [usize; 2],
) -> WgpuTensor<E, 4> {
    const WORKGROUP: usize = 32;

    let grad = kernel::into_contiguous(grad);

    let num_elems = x.shape.num_elements();
    let buffer = x
        .context
        .create_buffer(num_elems * core::mem::size_of::<E>());
    let output = WgpuTensor::new(x.context.clone(), x.shape.clone(), buffer);
    let info_buffer = build_pool2d_info(&x, &grad, kernel_size, stride, padding);
    let kernel = x
        .context
        .compile_static::<KernelSettings<AvgPool2dBackward, E, i32, WORKGROUP, WORKGROUP, 1>>();

    x.context.execute(
        elemwise_workgroup(output.shape.num_elements(), WORKGROUP),
        kernel,
        &[&grad.buffer, &output.buffer, &info_buffer],
    );

    output
}

#[cfg(test)]
mod tests {
    use crate::tests::{ReferenceBackend, TestBackend};
    use burn_tensor::{backend::Backend, module, ops::ModuleOps, Distribution, Tensor};

    #[test]
    fn avg_pool2d_should_work_with_multiple_invocations() {
        let tensor = Tensor::<TestBackend, 4>::random([32, 32, 32, 32], Distribution::Default);
        let tensor_ref = Tensor::<ReferenceBackend, 4>::from_data(tensor.to_data());
        let kernel_size = [3, 4];
        let stride = [1, 2];
        let padding = [1, 2];

        let pooled = module::avg_pool2d(tensor, kernel_size, stride, padding);
        let pooled_ref = module::avg_pool2d(tensor_ref, kernel_size, stride, padding);

        pooled
            .into_data()
            .assert_approx_eq(&pooled_ref.into_data(), 3);
    }

    #[test]
    fn avg_pool2d_backward_should_work_with_multiple_invocations() {
        TestBackend::seed(0);
        ReferenceBackend::seed(0);
        let tensor = Tensor::<TestBackend, 4>::random([32, 32, 32, 32], Distribution::Default);
        let tensor_ref = Tensor::<ReferenceBackend, 4>::from_data(tensor.to_data());
        let kernel_size = [3, 3];
        let stride = [1, 1];
        let padding = [1, 1];

        let shape_out = module::avg_pool2d(tensor.clone(), kernel_size, stride, padding).shape();
        let grad_output = Tensor::<TestBackend, 4>::random(shape_out, Distribution::Default);
        let grad_output_ref = Tensor::<ReferenceBackend, 4>::from_data(grad_output.to_data());

        let grad: Tensor<TestBackend, 4> =
            Tensor::from_primitive(TestBackend::avg_pool2d_backward(
                tensor.into_primitive(),
                grad_output.into_primitive(),
                kernel_size,
                stride,
                padding,
            ));
        let grad_ref: Tensor<ReferenceBackend, 4> =
            Tensor::from_primitive(ReferenceBackend::avg_pool2d_backward(
                tensor_ref.into_primitive(),
                grad_output_ref.into_primitive(),
                kernel_size,
                stride,
                padding,
            ));

        grad.into_data().assert_approx_eq(&grad_ref.into_data(), 3);
    }
}
