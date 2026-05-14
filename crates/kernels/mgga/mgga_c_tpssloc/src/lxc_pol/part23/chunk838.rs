//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 838/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk838<F: Float>(t1229: F, t3242: F, t11153: F, t3584: F, t1734: F, t3508: F, t1089: F, t475: F, t1744: F, t3540: F, t1731: F, t1706: F, t3545: F, t11818: F, t1735: F, t248: F) -> (F, F, F, F, F, F, F, F) {
    let t15615 = t1229 * t3242;
    let t15654 = t3584 * t11153;
    let t15659 = t1734 * t3508;
    let t15701 = t475 * t1089;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15727 = t1706 * t3545;
    let t15730 = t248 * t11818 * t1735;
    (t15615, t15654, t15659, t15701, t15717, t15719, t15727, t15730)
}
