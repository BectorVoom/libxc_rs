//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 712/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk712<F: Float>(t1942: F, t3082: F, t344: F, t40: F, t1009: F, t6740: F, t1015: F, t6746: F, t984: F, t1933: F, t225: F, t343: F, t364: F, t6721: F, t6739: F, t6741: F) -> (F, F, F, F, F) {
    let t23469 = t1942 * t3082 / 6912.0;
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    let t23472 = t6740 * t23471;
    let t23473 = t1015 * t6746;
    let t23474 = t23472 * t23473;
    let t23476 = t40 * t984;
    let t23477 = t1933 * t23476;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    let t23483 = t23482 * t6741;
    (t23469, t23474, t23476, t23480, t23483)
}
