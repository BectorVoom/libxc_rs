//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1116/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1116<F: Float>(t19818: F, t20047: F, t1006: F, t1398: F, t33: F, t3724: F, t1497: F, t750: F, t821: F, t4478: F, t7383: F, t18710: F, t6245: F, t19466: F, t19479: F, t19491: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20048 = t20047 * t19818;
    let t20050 = t1006 * t1398;
    let t20054 = t33 * t3724;
    let t20058 = t1497 * t750;
    let t20065 = t1497 * t821;
    let t20134 = t7383 * t4478;
    let t20137 = t18710 * t6245;
    let t20142 = 7.0 / 72.0 * t19466;
    let t20146 = 7.0 / 1152.0 * t19479;
    let t20151 = 7.0 / 288.0 * t19491;
    (t20048, t20050, t20054, t20058, t20065, t20134, t20137, t20142, t20146, t20151)
}
