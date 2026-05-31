//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1001/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1001<F: Float>(t2018: F, t26161: F, t26558: F, t3914: F, t23938: F, t6535: F, t26977: F, t22561: F, t7042: F, t114422: F, t111: F, t31699: F) -> (F, F, F, F, F, F) {
    let t115227 = F::cast_from(2.0_f64) * t26161 * t26558 * t2018 * t3914;
    let t115229 = F::cast_from(4.0_f64) * t23938 * t6535;
    let t115231 = F::cast_from(4.0_f64) * t26977 * t6535;
    let t115233 = F::cast_from(4.0_f64) * t7042 * t22561;
    let t115238 = F::cast_from(4.0_f64) * t26161 * t26558 * t114422;
    let t115241 = t31699 * t111;
    (t115227, t115229, t115231, t115233, t115238, t115241)
}
