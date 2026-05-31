//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1987/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1987<F: Float>(t13042: F, t13053: F, t16804: F, t2047: F, t259: F, t2597: F, t2713: F, t2718: F, t29055: F, t29056: F, t29080: F, t7830: F, t7842: F, t855: F, t865: F, t87929: F, t92966: F, t92976: F, t99033: F, t99036: F) -> F {
    let t101828 = -t92966 + F::cast_from(4.0_f64) * t2713 * t29080 - t87929 + t16804 * t2047 * t259 - F::cast_from(0.6579736267392905746e-1_f64) * t99033 + F::cast_from(0.3289868133696452873e-1_f64) * t99036 - t92976 + F::cast_from(4.0_f64) * t13053 * t7830 - F::cast_from(2.0_f64) * t13042 * t7842 + F::cast_from(2.0_f64) * t855 * t2718 * t29055 * t865 - t2597 * t29056;
    t101828
}
