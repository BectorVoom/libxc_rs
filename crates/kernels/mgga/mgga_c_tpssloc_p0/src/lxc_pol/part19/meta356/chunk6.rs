//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1293/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1293<F: Float>(t2928: F, t315: F, t2931: F, t10843: F, t923: F, t10744: F, t10750: F, t10760: F, t10765: F, t10771: F, t10825: F, t2861: F, t2881: F, t2886: F, t2888: F, t2907: F, t41827: F, t41987: F, t41998: F, t42002: F, t42005: F, t42011: F, t42020: F, t42025: F, t42031: F, t42097: F, t42105: F, t42106: F, t932: F, t933: F, t952: F) -> (F, F, F) {
    let t42109 = t2928 * t2928;
    let t42110 = F::cast_from(1.0_f64) / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = F::cast_from(1.0_f64) / t42112;
    let t42117 = t10843 * t923;
    let t42122 = t41998 + t42002 - t42005 + F::cast_from(24.0_f64) * t10765 * t10744 - F::cast_from(24.0_f64) * t10771 * t41987 * t932 - F::cast_from(6.0_f64) * t2861 * t42011 * t932 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t42011 * t2888 + F::cast_from(0.14035736694323150897e2_f64) * t10825 * t10750 - F::cast_from(0.70178683471615754484e1_f64) * t42020 * t2907 - t42025 + t42031 - t42097 - t42105 + F::cast_from(0.23392894490538584828e1_f64) * t42106 * t952 + F::cast_from(0.91082604192152556044e5_f64) * t42111 * t41827 * t42113 + F::cast_from(4.0_f64) * t42117 * t933 + F::cast_from(6.0_f64) * t10760 * t2881;
    (t42110, t42113, t42122)
}
