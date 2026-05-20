//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1965/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1965<F: Float>(t225: F, t29095: F, t26729: F, t866: F, t86930: F, t86931: F, t92415: F, t92425: F, t98202: F, t98205: F, t98213: F, t98222: F, t98227: F, t98279: F) -> F {
    let t101355 = t29095 * t225;
    let t101359 = -F::cast_from(0.3289868133696452873e-1_f64) * t98202 + F::cast_from(0.19739208802178717238e0_f64) * t98205 - F::new(12.0) * t98279 * t26729 - t92415 + t86930 - t86931 - F::cast_from(0.3289868133696452873e-1_f64) * t98213 - t101355 * t866 + F::cast_from(0.6579736267392905746e-1_f64) * t98222 - F::cast_from(0.9869604401089358619e-1_f64) * t98227 + t92425;
    t101359
}
