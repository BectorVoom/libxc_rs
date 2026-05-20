//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2391/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2391<F: Float>(t42086: F, t42087: F, t47781: F, t47785: F, t47787: F, t49043: F, t49049: F, t49052: F, t49054: F, t49056: F, t49058: F, t49060: F) -> F {
    let t49062 = F::cast_from(0.427258125e1_f64) * t49043 + t42086 + t42087 - F::cast_from(0.99655555555555555554e0_f64) * t47781 - F::new(0.53814e1) * t47785 + F::cast_from(0.31003950617283950619e0_f64) * t47787 - F::new(0.28483875e1) * t49049 + F::new(0.46074375e0) * t49052 + F::new(0.46074375e0) * t49054 + F::new(0.15358125e0) * t49056 - F::new(0.28483875e1) * t49058 + F::new(0.1898925e1) * t49060;
    t49062
}
