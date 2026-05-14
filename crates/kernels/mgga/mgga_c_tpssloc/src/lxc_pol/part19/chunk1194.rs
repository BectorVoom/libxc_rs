//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1194/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1194<F: Float>(t2840: F, t275: F, t2843: F, t41995: F, t10619: F, t942: F, t2928: F, t315: F, t2931: F, t10843: F, t923: F, t10744: F, t10750: F, t10760: F, t10765: F, t10771: F, t10825: F, t2861: F, t2881: F, t2886: F, t2888: F, t2907: F, t41827: F, t41987: F, t41998: F, t42002: F, t42005: F, t42011: F, t42020: F, t42025: F, t42031: F, t42097: F, t932: F, t933: F, t952: F) -> (F, F, F, F) {
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    let t42102 = 1.0 / t42101;
    let t42105 = 0.24955700379505800916e5 * t42100 * t41995 * t42102;
    let t42106 = t10619 * t942;
    let t42109 = t2928 * t2928;
    let t42110 = 1.0 / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = 1.0 / t42112;
    let t42117 = t10843 * t923;
    let t42122 = t41998 + t42002 - t42005 + 24.0 * t10765 * t10744 - 24.0 * t10771 * t41987 * t932 - 6.0 * t2861 * t42011 * t932 + 0.96491876992155210402e2 * t2886 * t42011 * t2888 + 0.14035736694323150897e2 * t10825 * t10750 - 0.70178683471615754484e1 * t42020 * t2907 - t42025 + t42031 - t42097 - t42105 + 0.23392894490538584828e1 * t42106 * t952 + 0.91082604192152556044e5 * t42111 * t41827 * t42113 + 4.0 * t42117 * t933 + 6.0 * t10760 * t2881;
    (t42105, t42110, t42113, t42122)
}
