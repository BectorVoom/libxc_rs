//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1239/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1239<F: Float>(t225: F, t25051: F, t1509: F, t6624: F, t1902: F, t4233: F, t25161: F, t214: F, t4265: F, t25222: F, t25220: F, t28: F, t40772: F) -> (F, F, F, F, F, F, F, F) {
    let t86988 = t25051 * t225;
    let t87567 = t6624 * t1509;
    let t87620 = t1902 * t4233;
    let t87758 = t25161 * t225;
    let t87782 = t214 * t4265;
    let t87810 = t25222 * t225;
    let t87837 = t25220 * t225;
    let t89953 = t40772 * t28;
    (t86988, t87567, t87620, t87758, t87782, t87810, t87837, t89953)
}
