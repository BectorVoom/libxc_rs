//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1132/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1132<F: Float>(t22674: F, t28191: F, t80681: F, t28206: F, t6883: F, t28205: F, t6897: F, t22892: F, t28209: F, t28117: F, t81159: F, t1377: F, t6330: F) -> (F, F, F, F, F, F) {
    let t96848 = t80681 * t22674 * t28191;
    let t96868 = t6883 * t28206;
    let t96878 = t6897 * t22674 * t28205;
    let t96893 = t22892 * t22674 * t28209;
    let t96920 = t81159 * t28117;
    let t96922 = t1377 * t6330;
    (t96848, t96868, t96878, t96893, t96920, t96922)
}
