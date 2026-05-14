//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 960/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk960<F: Float>(t11717: F, t3503: F, t11713: F, t11720: F, t3508: F, t1214: F, t248: F, t11708: F, t3514: F, t1210: F, t475: F, t3509: F, t3570: F, t3506: F, t11159: F, t3440: F) -> (F, F, F, F, F, F, F) {
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    let t11729 = t11720 * t3508;
    let t11731 = t248 * t1214 * t11729;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11739 = t11720 * t475;
    let t11741 = t248 * t1214 * t11739;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11748 = t3440 * t11159;
    (t11728, t11731, t11734, t11738, t11741, t11746, t11748)
}
