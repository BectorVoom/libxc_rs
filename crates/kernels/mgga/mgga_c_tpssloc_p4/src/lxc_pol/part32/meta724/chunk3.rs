//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2321/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321<F: Float>(t1222: F, t29601: F, t104107: F, t104111: F, t104120: F, t104124: F, t104126: F, t1232: F, t18383: F, t18965: F, t2136: F, t24736: F, t24741: F, t25588: F, t29625: F, t6207: F, t7316: F, t8027: F, t86191: F, t86327: F, t95370: F) -> F {
    let t104128 = t29601 * t1222;
    let t104134 = t86191 - t24736 * t6207 / F::new(2304.0) - F::new(19.0) / F::new(1296.0) * t104107 * t1232 + t95370 + F::cast_from(0.16149102437656156342e-2_f64) * t104111 + F::cast_from(0.16149102437656156342e-2_f64) * t8027 * t25588 * t2136 + F::cast_from(0.10093189023535097714e-3_f64) * t7316 * t29625 + F::cast_from(0.20186378047070195428e-3_f64) * t104120 - F::cast_from(0.10093189023535097714e-3_f64) * t104124 - t104126 / F::new(216.0) + F::new(19.0) / F::new(1296.0) * t104128 - t24741 * t18383 / F::new(2304.0) + t86327 * t18965 / F::new(2304.0);
    t104134
}
