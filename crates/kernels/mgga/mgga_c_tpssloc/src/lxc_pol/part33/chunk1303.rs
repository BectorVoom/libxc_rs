//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1303/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1303<F: Float>(t23384: F, t28660: F, t28614: F, t362: F, t5914: F, t28719: F, t3216: F, t112: F, t28868: F, t28904: F, t576: F, t580: F) -> (F, F, F, F, F, F, F) {
    let t100431 = t23384 * t28660;
    let t100436 = t23384 * t28614;
    let t100449 = t362 * t5914;
    let t100497 = t28719 * t3216;
    let t100911 = t28868 * t112;
    let t100945 = t576 * t28904;
    let t100946 = t28868 * t580;
    (t100431, t100436, t100449, t100497, t100911, t100945, t100946)
}
