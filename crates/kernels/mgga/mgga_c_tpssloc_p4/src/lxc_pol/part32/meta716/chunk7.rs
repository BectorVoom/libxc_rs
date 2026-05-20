//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2266/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2266<F: Float>(t23168: F, t28277: F, t28295: F, t6547: F, t6562: F, t7488: F, t86893: F, t28439: F, t28268: F, t81591: F, t17049: F, t1880: F, t6553: F, t6571: F) -> (F, F, F, F, F, F) {
    let t98921 = t23168 * t28277;
    let t98923 = t6547 * t28295;
    let t98927 = t6562 * t86893 * t7488;
    let t98932 = t6547 * t28439;
    let t98941 = t81591 * t28268;
    let t98945 = t1880 * t6553 * t6571 * t17049;
    (t98921, t98923, t98927, t98932, t98941, t98945)
}
