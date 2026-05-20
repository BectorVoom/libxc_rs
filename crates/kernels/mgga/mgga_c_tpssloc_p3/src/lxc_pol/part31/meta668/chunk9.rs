//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1974/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1974<F: Float>(t101398: F, t101413: F, t101425: F, t101439: F, t101456: F, t101468: F, t101486: F, t101496: F, t1528: F, t17056: F, t218: F, t25168: F, t259: F, t26728: F, t2713: F, t29091: F, t86983: F, t86991: F, t86994: F, t92386: F, t98251: F, t98256: F, t98264: F, t98277: F) -> (F, F) {
    let t101499 = t101398 + t101413 + t101425 + t101439 + t101456 + t101468 + t101486 + t101496;
    let t101504 = -F::new(6.0) * t2713 * t29091 + F::cast_from(0.6579736267392905746e-1_f64) * t98251 + F::cast_from(0.3289868133696452873e-1_f64) * t98256 + F::cast_from(0.6579736267392905746e-1_f64) * t98264 - F::new(2.0) * t92386 * t1528 + t86983 - F::new(6.0) * t25168 * t26728 * t17056 + t218 * t101499 * t259 - F::cast_from(0.13159472534785811492e0_f64) * t98277 - F::cast_from(0.25587863262083522345e0_f64) * t86991 + t86994;
    (t101499, t101504)
}
