//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1183/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1183<F: Float>(t113: F, t1266: F, t2165: F, t2167: F, t22460: F, t22467: F, t22482: F, t22563: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t24543: F, t24545: F, t24552: F, t24924: F, t24932: F, t24935: F, t24939: F, t3929: F, t510: F, t574: F, t650: F, t652: F, t672: F, t7264: F, t7266: F, t7271: F, t7408: F) -> F {
    let t24949 = -t113 * t24924 - F::cast_from(2.0_f64) * t1266 * t7264 - t2165 * t2312 - F::cast_from(2.0_f64) * t2165 * t2320 + t2167 * t3929 - F::cast_from(4.0_f64) * t2314 * t7271 - F::cast_from(4.0_f64) * t2323 * t7266 - F::cast_from(2.0_f64) * t2364 * t7266 - t24543 * t510 - F::cast_from(4.0_f64) * t24545 * t652 - F::cast_from(2.0_f64) * t24552 * t652 - F::cast_from(4.0_f64) * t24932 * t672 - F::cast_from(2.0_f64) * t24935 * t510 + t24939 * t574 - F::cast_from(2.0_f64) * t650 * t7408 - t22460 - t22467 - t22482 - t22563;
    t24949
}
