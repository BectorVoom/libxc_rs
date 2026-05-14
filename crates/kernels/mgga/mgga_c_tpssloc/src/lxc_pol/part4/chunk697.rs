//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 697/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk697<F: Float>(t5210: F, t562: F, t1372: F, t1807: F, t1808: F, t225: F, t1323: F, t1834: F, t1811: F, t3726: F, t1307: F, t1810: F, t210: F, t119: F, t5187: F, t554: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5211 = t5210 * t562;
    let t5213 = t1807 * t1372;
    let t5215 = t1808 * t225;
    let t5217 = t1323 * t1834;
    let t5220 = t3726 * t1811;
    let t5223 = t210 * t1810 * t1307;
    let t5226 = t119 * t5187;
    let t5227 = t210 * t5226;
    let t5230 = t5210 * t225;
    let t5231 = t5230 * t554;
    (t5211, t5213, t5215, t5217, t5220, t5223, t5227, t5230, t5231)
}
