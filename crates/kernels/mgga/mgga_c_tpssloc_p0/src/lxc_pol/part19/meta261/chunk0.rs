//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1009/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1009<F: Float>(t11708: F, t3505: F, t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F, t1215: F, t3507: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11709 = t11708 * t3505;
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    let t11714 = t1208 * t1208;
    let t11715 = F::new(1.0) / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11720 = t3507 * t1215;
    (t11709, t11712, t11713, t11714, t11715, t11716, t11717, t11718, t11719, t11720)
}
