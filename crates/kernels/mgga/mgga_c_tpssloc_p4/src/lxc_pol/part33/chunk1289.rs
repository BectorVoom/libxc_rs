//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1289/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1289<F: Float>(t6562: F, t7488: F, t86893: F, t28439: F, t6547: F, t28268: F, t81591: F, t23204: F, t28294: F, t28263: F, t28299: F, t81979: F) -> (F, F, F, F, F, F) {
    let t98927 = t6562 * t86893 * t7488;
    let t98932 = t6547 * t28439;
    let t98941 = t81591 * t28268;
    let t98966 = t6562 * t23204 * t28294;
    let t98983 = t6562 * t23204 * t28263;
    let t98993 = t81979 * t28299;
    (t98927, t98932, t98941, t98966, t98983, t98993)
}
