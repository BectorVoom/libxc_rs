//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1254/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1254<F: Float>(t1902: F, t4233: F, t254: F, t799: F, t225: F, t25161: F, t214: F, t4265: F, t25222: F, t25220: F, t10143: F, t1081: F) -> (F, F, F, F, F, F, F) {
    let t87620 = t1902 * t4233;
    let t87755 = t799 * t254;
    let t87758 = t25161 * t225;
    let t87782 = t214 * t4265;
    let t87810 = t25222 * t225;
    let t87837 = t25220 * t225;
    let t89849 = t10143 * t1081;
    (t87620, t87755, t87758, t87782, t87810, t87837, t89849)
}
