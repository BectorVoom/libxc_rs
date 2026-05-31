//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1373/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1373<F: Float>(t11761: F, t11850: F, t11858: F, t2140: F, t24699: F, t24749: F, t488: F, t7310: F, t7316: F, t7321: F, t86348: F, t86350: F, t86354: F, t86357: F, t86365: F, t86368: F) -> F {
    let t86373 = t86348 / F::cast_from(3456.0_f64) - t86350 / F::cast_from(2304.0_f64) + t7310 * t11761 / F::cast_from(36.0_f64) - t86354 / F::cast_from(576.0_f64) - F::cast_from(0.30279567070605293142e-3_f64) * t86357 - t7310 * t11850 / F::cast_from(48.0_f64) + F::cast_from(0.30279567070605293142e-3_f64) * t24749 * t7321 + F::cast_from(0.30279567070605293142e-3_f64) * t7316 * t24699 + t86365 / F::cast_from(216.0_f64) - F::cast_from(0.30279567070605293142e-3_f64) * t86368 + t11858 * t2140 * t488 / F::cast_from(1536.0_f64);
    t86373
}
