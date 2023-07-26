// Language
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::Range;

// Current crate
use super::{matmul::matmul, NdArrayMathOps, NdArrayOps};
use crate::element::FloatNdArrayElement;
use crate::{tensor::NdArrayTensor, NdArrayBackend};
use crate::{NdArrayDevice, SEED};

// Workspace crates
use burn_common::rand::get_seeded_rng;
use burn_tensor::Distribution;
use burn_tensor::{backend::Backend, ops::TensorOps, Data, ElementConversion, Shape};

// External crates
use libm::{cos, erf, sin, tanh};

#[cfg(not(feature = "std"))]
use num_traits::Float;

impl<E: FloatNdArrayElement> TensorOps<NdArrayBackend<E>> for NdArrayBackend<E> {
    fn from_data<const D: usize>(data: Data<E, D>, _device: &NdArrayDevice) -> NdArrayTensor<E, D> {
        NdArrayTensor::from_data(data)
    }

    fn random<const D: usize>(
        shape: Shape<D>,
        distribution: Distribution<E>,
        device: &NdArrayDevice,
    ) -> NdArrayTensor<E, D> {
        let mut seed = SEED.lock().unwrap();
        let mut rng = if let Some(rng_seeded) = seed.as_ref() {
            rng_seeded.clone()
        } else {
            get_seeded_rng()
        };
        let tensor = Self::from_data(Data::random(shape, distribution, &mut rng), device);
        *seed = Some(rng);
        tensor
    }

    fn shape<const D: usize>(tensor: &NdArrayTensor<E, D>) -> Shape<D> {
        tensor.shape()
    }

    fn to_data<const D: usize>(
        tensor: &NdArrayTensor<E, D>,
    ) -> Data<<NdArrayBackend<E> as Backend>::FloatElem, D> {
        let values = tensor.array.iter().map(Clone::clone).collect();
        Data::new(values, tensor.shape())
    }

    fn into_data<const D: usize>(
        tensor: NdArrayTensor<E, D>,
    ) -> Data<<NdArrayBackend<E> as Backend>::FloatElem, D> {
        let shape = tensor.shape();
        let values = tensor.array.into_iter().collect();
        Data::new(values, shape)
    }

    fn device<const D: usize>(_tensor: &NdArrayTensor<E, D>) -> NdArrayDevice {
        NdArrayDevice::Cpu
    }

    fn to_device<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        _device: &NdArrayDevice,
    ) -> NdArrayTensor<E, D> {
        tensor
    }

    fn empty<const D: usize>(
        shape: Shape<D>,
        device: &<NdArrayBackend<E> as Backend>::Device,
    ) -> NdArrayTensor<E, D> {
        NdArrayBackend::<E>::zeros(shape, device)
    }

    fn add<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::add(lhs, rhs)
    }

    fn add_scalar<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<E, D> {
        NdArrayMathOps::add_scalar(lhs, rhs)
    }

    fn sub<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::sub(lhs, rhs)
    }

    fn sub_scalar<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<E, D> {
        NdArrayMathOps::sub_scalar(lhs, rhs)
    }

    fn mul<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::mul(lhs, rhs)
    }

    fn mul_scalar<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<E, D> {
        NdArrayMathOps::mul_scalar(lhs, rhs)
    }

    fn div<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::div(lhs, rhs)
    }

    fn div_scalar<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<E, D> {
        NdArrayMathOps::div_scalar(lhs, rhs)
    }

    fn matmul<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        matmul(lhs, rhs)
    }

    fn neg<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        Self::mul_scalar(tensor, (-1f32).elem::<E>())
    }

    fn swap_dims<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        dim1: usize,
        dim2: usize,
    ) -> NdArrayTensor<E, D> {
        let mut array = tensor.array;
        array.swap_axes(dim1, dim2);

        NdArrayTensor::new(array)
    }

    fn reshape<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<E, D1>,
        shape: Shape<D2>,
    ) -> NdArrayTensor<E, D2> {
        NdArrayOps::reshape(tensor, shape)
    }

    fn index_select<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        indexes: NdArrayTensor<i64, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::index_select(tensor, indexes)
    }

    fn index_select_assign<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        indexes: NdArrayTensor<i64, D>,
        value: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::index_select_assign(tensor, indexes, value)
    }

    fn index_select_dim<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        dim: usize,
        indexes: NdArrayTensor<i64, 1>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::index_select_dim(tensor, dim, indexes)
    }

    fn index_select_dim_assign<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<E, D1>,
        dim: usize,
        indexes: NdArrayTensor<i64, 1>,
        value: NdArrayTensor<E, D2>,
    ) -> NdArrayTensor<E, D1> {
        NdArrayMathOps::index_select_dim_assign(tensor, dim, indexes, value)
    }

    fn index<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<E, D1>,
        indexes: [Range<usize>; D2],
    ) -> NdArrayTensor<E, D1> {
        NdArrayOps::index(tensor, indexes)
    }

    fn index_assign<const D1: usize, const D2: usize>(
        tensor: NdArrayTensor<E, D1>,
        indexes: [Range<usize>; D2],
        value: NdArrayTensor<E, D1>,
    ) -> NdArrayTensor<E, D1> {
        NdArrayOps::index_assign(tensor, indexes, value)
    }

    fn mask_scatter<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        mask: NdArrayTensor<bool, D>,
        source: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::mask_scatter(tensor, mask, source)
    }

    fn mask_fill<const D: usize>(
        tensor: NdArrayTensor<E, D>,
        mask: NdArrayTensor<bool, D>,
        value: E,
    ) -> NdArrayTensor<E, D> {
        NdArrayMathOps::mask_fill(tensor, mask, value)
    }

    fn equal<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<bool, D> {
        let tensor = NdArrayBackend::<E>::sub(lhs, rhs);
        let zero = 0.elem();

        Self::equal_elem(tensor, zero)
    }

    fn equal_elem<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<bool, D> {
        let array = lhs.array.mapv(|a| a == rhs).into_shared();

        NdArrayTensor::new(array)
    }

    fn greater<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<bool, D> {
        let tensor = NdArrayBackend::<E>::sub(lhs, rhs);
        let zero = 0.elem();
        Self::greater_elem(tensor, zero)
    }

    fn greater_elem<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<bool, D> {
        let array = lhs.array.mapv(|a| a > rhs).into_shared();

        NdArrayTensor::new(array)
    }

    fn greater_equal<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<bool, D> {
        let tensor = NdArrayBackend::<E>::sub(lhs, rhs);
        let zero = 0.elem();
        Self::greater_equal_elem(tensor, zero)
    }

    fn greater_equal_elem<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: E,
    ) -> NdArrayTensor<bool, D> {
        let array = lhs.array.mapv(|a| a >= rhs).into_shared();

        NdArrayTensor::new(array)
    }

    fn lower<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<bool, D> {
        let tensor = NdArrayBackend::<E>::sub(lhs, rhs);
        let zero = 0.elem();
        Self::lower_elem(tensor, zero)
    }

    fn lower_elem<const D: usize>(lhs: NdArrayTensor<E, D>, rhs: E) -> NdArrayTensor<bool, D> {
        let array = lhs.array.mapv(|a| a < rhs).into_shared();

        NdArrayTensor::new(array)
    }

    fn lower_equal<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<bool, D> {
        let tensor = NdArrayBackend::<E>::sub(lhs, rhs);
        let zero = 0.elem();
        Self::lower_equal_elem(tensor, zero)
    }

    fn lower_equal_elem<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: E,
    ) -> NdArrayTensor<bool, D> {
        let array = lhs.array.mapv(|a| a <= rhs).into_shared();

        NdArrayTensor::new(array)
    }

    fn detach<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        tensor
    }

    fn maximum<const D: usize>(
        lhs: NdArrayTensor<E, D>,
        rhs: NdArrayTensor<E, D>,
    ) -> NdArrayTensor<E, D> {
        // let array = lhs.array(rhs.array).mapv(|(a, b)| a.max(b));
        // NdArrayTensor::new(array)
        todo!()
    }

    fn mean<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, 1> {
        NdArrayMathOps::mean(tensor)
    }

    fn sum<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, 1> {
        NdArrayMathOps::sum(tensor)
    }

    fn mean_dim<const D: usize>(tensor: NdArrayTensor<E, D>, dim: usize) -> NdArrayTensor<E, D> {
        NdArrayMathOps::mean_dim(tensor, dim)
    }

    fn sum_dim<const D: usize>(tensor: NdArrayTensor<E, D>, dim: usize) -> NdArrayTensor<E, D> {
        NdArrayMathOps::sum_dim(tensor, dim)
    }

    fn to_full_precision<const D: usize>(tensor: &NdArrayTensor<E, D>) -> NdArrayTensor<f32, D> {
        let array = tensor.array.mapv(|a| a.elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn from_full_precision<const D: usize>(tensor: NdArrayTensor<f32, D>) -> NdArrayTensor<E, D> {
        let array = tensor.array.mapv(|a| a.elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn argmax<const D: usize>(tensor: NdArrayTensor<E, D>, dim: usize) -> NdArrayTensor<i64, D> {
        arg(tensor, dim, cmp_min)
    }

    fn argmin<const D: usize>(tensor: NdArrayTensor<E, D>, dim: usize) -> NdArrayTensor<i64, D> {
        arg(tensor, dim, cmp_max)
    }

    fn exp<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor.array.mapv_into(|a| a.exp_elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn log<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor.array.mapv_into(|a| a.log_elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn log1p<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor.array.mapv_into(|a| a.log1p_elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn powf<const D: usize>(tensor: NdArrayTensor<E, D>, value: f32) -> NdArrayTensor<E, D> {
        let array = if value == 2.0 {
            // Happens often and is faster.
            tensor.array.mapv_into(|a| a * a).into_shared()
        } else if value.floor() == value {
            // Is faster then powf
            tensor
                .array
                .mapv_into(|a| a.powi_elem(value as i32))
                .into_shared()
        } else {
            // Default
            tensor.array.mapv_into(|a| a.powf_elem(value)).into_shared()
        };

        NdArrayTensor::new(array)
    }

    fn sqrt<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor.array.mapv_into(|a| a.sqrt_elem()).into_shared();

        NdArrayTensor::new(array)
    }

    fn cos<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor
            .array
            .mapv_into(|a| cos(a.to_f64().unwrap()).elem())
            .into_shared();

        NdArrayTensor::new(array)
    }

    fn sin<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor
            .array
            .mapv_into(|a| sin(a.to_f64().unwrap()).elem())
            .into_shared();

        NdArrayTensor::new(array)
    }

    fn tanh<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor
            .array
            .mapv_into(|a| tanh(a.to_f64().unwrap()).elem())
            .into_shared();

        NdArrayTensor::new(array)
    }

    fn erf<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let array = tensor
            .array
            .mapv_into(|a| erf(a.to_f64().unwrap()).elem())
            .into_shared();

        NdArrayTensor::new(array)
    }

    fn cat<const D: usize>(tensors: Vec<NdArrayTensor<E, D>>, dim: usize) -> NdArrayTensor<E, D> {
        NdArrayOps::cat(tensors, dim)
    }

    fn relu<const D: usize>(tensor: NdArrayTensor<E, D>) -> NdArrayTensor<E, D> {
        let zero = 0.elem();
        let array = tensor
            .array
            .mapv_into(|elem| match elem < zero {
                true => 0.0.elem(),
                false => elem,
            })
            .into_shared();

        NdArrayTensor::new(array)
    }
}

fn arg<E: FloatNdArrayElement, F, const D: usize>(
    tensor: NdArrayTensor<E, D>,
    dim: usize,
    cmp: F,
) -> NdArrayTensor<i64, D>
where
    F: Fn(&f64, &f64) -> Ordering,
{
    let batch_size = tensor.shape().dims[dim];

    let mut data = NdArrayBackend::into_data::<D>(tensor.clone());
    let mut start = 0;
    let mut end = tensor.shape().dims[dim];
    let mut output = Vec::new();

    while end <= data.value.len() {
        let data_dim = &mut data.value[start..end];
        let mut sorted: Vec<f64> = data_dim.iter().map(|a| a.elem()).collect();
        sorted.sort_by(&cmp);

        let max = sorted[0];

        let data_dim = &mut data.value[start..end];
        let mut index: i64 = 0;
        for elem in data_dim {
            let as_float: f64 = elem.elem();
            if as_float == max {
                break;
            }
            index += 1;
        }
        output.push(index);
        start += batch_size;
        end += batch_size;
    }
    let mut shape = tensor.shape();
    shape.dims[dim] = 1;
    NdArrayTensor::from_data(Data::new(output, shape))
}

fn cmp_max(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn cmp_min(a: &f64, b: &f64) -> Ordering {
    if a > b {
        return Ordering::Less;
    } else if a < b {
        return Ordering::Greater;
    }
    Ordering::Equal
}
