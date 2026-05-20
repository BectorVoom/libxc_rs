//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2611/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2611<F: Float>(t1222: F, t15765: F, t3242: F, t3448: F, t11728: F, t13969: F, t15630: F, t11718: F, t52835: F, t11797: F, t5024: F, t11147: F, t15394: F) -> (F, F, F, F, F, F) {
    let t53185 = t15765 * t1222;
    let t53187 = t3448 * t3242;
    let t53220 = t11728 * t13969 * t15630;
    let t53238 = t52835 * t11718;
    let t53246 = t5024 * t11797;
    let t53249 = t15394 * t11147;
    (t53185, t53187, t53220, t53238, t53246, t53249)
}
