//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 903/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk903<F: Float>(t225: F, t28051: F, t2006: F, t6387: F, t6414: F, t1824: F, t7722: F, t214: F, t6434: F, t28108: F, t1808: F, t254: F) -> (F, F, F, F, F, F, F) {
    let t96913 = t28051 * t225;
    let t97172 = t2006 * t6387;
    let t97181 = t2006 * t6414;
    let t97189 = t7722 * t1824;
    let t97511 = t214 * t6434;
    let t97558 = t28108 * t225;
    let t97626 = t1808 * t254;
    (t96913, t97172, t97181, t97189, t97511, t97558, t97626)
}
