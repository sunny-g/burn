use crate as burn;

use crate::config::Config;
use crate::record::Record;
use crate::tensor::{ElementConversion, Tensor};
use burn_tensor::backend::Backend;

/// Configuration to create momentum [Momentum](Momentum).
#[derive(Config)]
pub struct MomentumConfig {
    /// Momemtum factor
    #[config(default = 0.9)]
    pub momentum: f64,
    /// Dampening factor.
    #[config(default = 0.1)]
    pub dampening: f64,
    /// Enables Nesterov momentum, see [On the importance of initialization and
    /// momentum in deep learning](http://www.cs.toronto.edu/~hinton/absps/momentum.pdf).
    #[config(default = false)]
    pub nesterov: bool,
}

/// State of [Momentum](Momentum).
#[derive(Record, Clone, new)]
pub struct MomemtumState<B: Backend, const D: usize> {
    velocity: Tensor<B, D>,
}

/// Momemtum implementation that transforms gradients.
pub struct Momentum<B: Backend> {
    momentum: B::FloatElem,
    dampening: f64,
    nesterov: bool,
}

impl<B: Backend> Momentum<B> {
    /// Creates a new [Momentum](Momentum) from a [MomentumConfig](MomentumConfig).
    pub fn new(config: &MomentumConfig) -> Self {
        Self {
            momentum: config.momentum.elem(),
            dampening: config.dampening,
            nesterov: config.nesterov,
        }
    }

    /// Transforms a gradient.
    ///
    /// # Arguments
    ///
    /// * `grad` - Gradient to transform.
    /// * `state` - State of the optimizer.
    ///
    /// # Returns
    ///
    /// * `grad` - Transformed gradient.
    /// * `state` - State of the optimizer.
    pub fn transform<const D: usize>(
        &self,
        grad: Tensor<B, D>,
        state: Option<MomemtumState<B, D>>,
    ) -> (Tensor<B, D>, MomemtumState<B, D>) {
        let velocity = if let Some(state) = state {
            grad.clone()
                .mul_scalar(1.0 - self.dampening)
                .add(state.velocity.mul_scalar(self.momentum))
        } else {
            grad.clone()
        };

        let grad = match self.nesterov {
            true => velocity.clone().mul_scalar(self.momentum).add(grad),
            false => velocity.clone(),
        };

        (grad, MomemtumState::new(velocity))
    }
}

impl<B: Backend, const D: usize> MomemtumState<B, D> {
    /// Moves the state to a device.
    ///
    /// # Arguments
    ///
    /// * `device` - Device to move the state to.
    ///
    /// # Returns
    ///
    /// * `self` - Moved state.
    pub fn to_device(mut self, device: &B::Device) -> Self {
        self.velocity = self.velocity.to_device(device);
        self
    }
}
