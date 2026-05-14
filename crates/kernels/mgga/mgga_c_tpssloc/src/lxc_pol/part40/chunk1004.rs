//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1004/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1004<F: Float>(t17635: F, t4583: F, t4582: F, t1041: F, t13948: F, t13952: F, t13959: F, t13963: F, t13966: F, t13972: F, t17616: F, t17621: F, t17625: F, t17632: F, t2960: F, t3039: F, t5885: F, t5890: F, t5894: F) -> (F,) {
    let t17636 = t4583 * t17635;
    let t17637 = t4582 * t17636;
    let t17640 = t17616 / 864.0 - t2960 * t5894 / 81.0 + t17621 / 648.0 + t13948 + t13952 + t13959 + t13963 - t13966 / 6912.0 - t17625 / 432.0 - t2960 * t5890 / 108.0 + t2960 * t5885 / 54.0 - t3039 * t17632 / 1536.0 - t1041 * t17637 / 2304.0 - t13972;
    (t17640,)
}
