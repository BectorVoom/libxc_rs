//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 840/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk840<F: Float>(t15501: F, t3503: F, t3500: F, t1210: F, t1742: F, t478: F, t3068: F, t1244: F, t1734: F, t3508: F, t1744: F, t3540: F, t1731: F, t1706: F, t3545: F, t11818: F, t1735: F, t248: F) -> (F, F, F, F, F, F, F, F) {
    let t15502 = t3503 * t15501;
    let t15503 = t3500 * t15502;
    let t15506 = t1210 * t15501;
    let t15507 = t3500 * t15506;
    let t15567 = t478 * t1742;
    let t15568 = t15567 * t3068;
    let t15569 = t1244 * t15568;
    let t15659 = t1734 * t3508;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15727 = t1706 * t3545;
    let t15730 = t248 * t11818 * t1735;
    (t15503, t15507, t15569, t15659, t15717, t15719, t15727, t15730)
}
