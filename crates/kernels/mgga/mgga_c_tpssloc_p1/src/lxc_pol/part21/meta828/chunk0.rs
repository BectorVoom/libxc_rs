//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2920/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2920<F: Float>(t17299: F, t2940: F, t13659: F, t4483: F, t17947: F, t2907: F, t959: F, t17191: F, t300: F, t961: F, t13724: F, t17564: F, t42671: F) -> (F, F, F, F, F, F) {
    let t60842 = F::cast_from(0.11696447245269292414e1_f64) * t2940 * t17299;
    let t60844 = F::cast_from(0.69263436422725855034e2_f64) * t4483 * t13659;
    let t60847 = F::cast_from(0.14035736694323150897e2_f64) * t959 * t17947 * t2907;
    let t60848 = t300 * t17191;
    let t60850 = F::cast_from(0.11696447245269292414e1_f64) * t60848 * t961;
    let t60852 = F::cast_from(0.2077903092681775651e3_f64) * t4483 * t13724;
    let t60855 = F::cast_from(0.10254018858216406658e4_f64) * t959 * t17564 * t42671;
    (t60842, t60844, t60847, t60850, t60852, t60855)
}
