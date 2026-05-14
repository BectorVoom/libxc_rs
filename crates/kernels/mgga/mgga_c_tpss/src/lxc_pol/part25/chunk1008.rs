//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1008/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1008<F: Float>(t11621: F, t14906: F, t3931: F, t1289: F, t3431: F, t3977: F, t3932: F, t3950: F, t361: F, t4977: F, t949: F, t14457: F, t3919: F, t14464: F, t11535: F, t14469: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14907 = t11621 * t14906;
    let t14908 = t3931 * t14907;
    let t14911 = t1289 * t3431;
    let t14912 = t3977 * t14911;
    let t14913 = t3931 * t14912;
    let t14916 = t3932 * t3950;
    let t14917 = t3931 * t14916;
    let t14920 = t361 * t4977;
    let t14921 = t14920 * t949;
    let t14922 = t3931 * t14921;
    let t14925 = t3919 * t14457;
    let t14928 = t3919 * t14464;
    let t14931 = t11535 * t14469;
    (t14908, t14911, t14913, t14917, t14920, t14922, t14925, t14928, t14931)
}
