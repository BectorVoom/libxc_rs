//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 981/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk981<F: Float>(t23012: F, t8538: F, t31339: F, t81591: F, t10049: F, t112687: F, t112697: F, t112700: F, t112703: F, t114632: F, t114648: F, t114668: F, t114695: F, t114754: F, t1911: F, t2054: F, t22975: F, t22979: F, t24281: F, t24282: F, t24305: F, t24314: F, t2713: F, t2718: F, t31343: F, t6627: F, t6663: F, t7087: F, t82287: F, t855: F, t8553: F, t858: F) -> F {
    let t114759 = t23012 * t8538;
    let t114760 = F::new(0.63969658155208805863e-1) * t114759;
    let t114762 = t81591 * t31339;
    let t114764 = -t112687 + F::new(4.0) * t2713 * t31343 + F::new(2.0) * t10049 * t8553 + F::new(2.0) * t7087 * t22975 - F::new(6.0) * t6627 * t24314 + F::new(2.0) * t855 * t2718 * t24281 * t1911 + F::new(0.16449340668482264365e-1) * t114632 - t112697 + t112700 - t112703 - F::new(2.0) * t24305 * t6663 + F::new(4.0) * t7087 * t22979 - F::new(2.0) * t82287 * t2054 - t855 * t858 * (t114648 + t114668 + t114695 + t114754) + t114760 - t6627 * t24282 - F::new(0.76763589786250567036e-1) * t114762;
    t114764
}
