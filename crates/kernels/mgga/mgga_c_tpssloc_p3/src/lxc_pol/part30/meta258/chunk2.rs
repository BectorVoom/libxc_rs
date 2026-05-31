//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1174/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1174<F: Float>(t1000: F, t1025: F, t1046: F, t1935: F, t1937: F, t350: F, t378: F, t6712: F, t6716: F, t6717: F, t6723: F, t6728: F, t6730: F, t6735: F, t6742: F, t6747: F, t6750: F, t6755: F, t6759: F, t6763: F, t6765: F) -> F {
    let t6768 = -t6712 * t350 / F::cast_from(36.0_f64) + t6716 + t6717 * t1000 / F::cast_from(288.0_f64) - F::cast_from(0.80745512188280781712e-3_f64) * t6723 * t1937 + t6728 + F::cast_from(0.10093189023535097714e-3_f64) * t6730 * t1937 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t6735 + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t6747 + t6750 * t378 / F::cast_from(1536.0_f64) + t6755 * t1025 / F::cast_from(1536.0_f64) - t6759 * t378 / F::cast_from(288.0_f64) + t6763 + t6765 * t1046 / F::cast_from(2304.0_f64);
    t6768
}
