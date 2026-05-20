//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1990/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1990<F: Float>(t2054: F, t24297: F, t26690: F, t2713: F, t4301: F, t46508: F, t82143: F, t82145: F, t82147: F, t82150: F, t855: F, t858: F, t87033: F, t87039: F, t92486: F, t92506: F, t92528: F, t92558: F, t92732: F, t92759: F, t92782: F, t92803: F, t92826: F) -> F {
    let t92839 = -F::new(2.0) * t24297 * t4301 + t92486 + F::cast_from(0.38381794893125283518e-1_f64) * t82143 - F::cast_from(0.3289868133696452873e-1_f64) * t87033 - t855 * t858 * (t92506 + t92528 + t92558 + t92732 + t92759 + t92782 + t92803 + t92826) - t46508 * t2054 - F::cast_from(0.13159472534785811492e0_f64) * t87039 + F::cast_from(0.76763589786250567036e-1_f64) * t82145 - F::cast_from(0.10417915756705434098e0_f64) * t82147 + F::cast_from(0.76763589786250567036e-1_f64) * t82150 + F::new(4.0) * t2713 * t26690;
    t92839
}
