//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1877/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1877<F: Float>(t1825: F, t22633: F, t6976: F, t90818: F, t26421: F, t5287: F, t22751: F, t28149: F, t19740: F, t1992: F, t22897: F, t28139: F) -> (F, F, F, F, F) {
    let t97087 = t22633 * t6976 * t90818 * t1825;
    let t97091 = t22633 * t6976 * t26421 * t5287;
    let t97095 = t22751 * t28149;
    let t97106 = t1992 * t22897 * t19740;
    let t97108 = t22751 * t28139;
    (t97087, t97091, t97095, t97106, t97108)
}
