//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 993/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk993<F: Float>(t12241: F, t1992: F, t22897: F, t22704: F, t22898: F, t80798: F, t12248: F, t6604: F, t12177: F, t562: F, t12250: F, t22720: F, t6883: F, t22716: F, t6983: F, t22742: F, t6914: F) -> (F, F, F, F, F, F, F) {
    let t81019 = t1992 * t22897 * t12241;
    let t81022 = t22704 * t80798 * t22898;
    let t81027 = t6604 * t12248;
    let t81028 = t562 * t12177;
    let t81031 = t1992 * t81027 * t81028 * t12250;
    let t81037 = t6883 * t22720;
    let t81039 = t22716 * t6983;
    let t81041 = t6914 * t22742;
    (t81019, t81022, t81028, t81031, t81037, t81039, t81041)
}
