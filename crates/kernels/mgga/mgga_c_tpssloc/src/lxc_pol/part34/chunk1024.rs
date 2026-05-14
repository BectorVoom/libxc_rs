//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1024/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1024<F: Float>(t1851: F, t7961: F, t112: F, t29395: F, t29376: F, t532: F, t2752: F, t29105: F, t225: F, t29095: F, t29099: F, t29071: F, t29040: F, t814: F, t2047: F, t5611: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100972 = t1851 * t7961;
    let t100996 = t29395 * t112;
    let t101150 = t532 * t29376;
    let t101226 = t29105 * t2752;
    let t101355 = t29095 * t225;
    let t101509 = t29099 * t225;
    let t101593 = t29071 * t225;
    let t101694 = t814 * t29040;
    let t101708 = t2047 * t5611;
    (t100972, t100996, t101150, t101226, t101355, t101509, t101593, t101694, t101708)
}
