//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 716/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk716<F: Float>(t1561: F, t2860: F, t1573: F, t2929: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t3199: F, t3185: F, t1654: F, t2394: F) -> (F, F, F, F, F, F, F) {
    let t14276 = t1561 * t2860;
    let t14337 = t1573 * t2929;
    let t14506 = t1603 * t3030;
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14608 = t14506 * t3199;
    let t14618 = t14506 * t3185;
    let t14702 = t2394 * t1654;
    (t14276, t14337, t14508, t14511, t14608, t14618, t14702)
}
