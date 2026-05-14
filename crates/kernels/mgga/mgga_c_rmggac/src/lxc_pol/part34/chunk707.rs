//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 707/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk707<F: Float>(t21719: F, t35155: F, t9217: F, t7248: F, t9105: F, t9110: F, t15231: F, t68432: F, t68386: F, t9117: F, t9188: F, t21708: F, t21714: F, t9183: F, t14025: F, t9187: F) -> (F, F, F, F, F, F, F) {
    let t74687 = t21719 * t35155 * t9217;
    let t74690 = t21719 * t7248 * t9105;
    let t74693 = t21719 * t7248 * t9110;
    let t74695 = t68432 * t15231;
    let t74698 = t68386 * t9188 * t9117;
    let t74701 = t21708 * t21714 * t9183;
    let t74703 = t14025 * t9187;
    (t74687, t74690, t74693, t74695, t74698, t74701, t74703)
}
