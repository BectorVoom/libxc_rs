//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2430/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430<F: Float>(t21360: F, t923: F, t10756: F, t10765: F, t10828: F, t14263: F, t14337: F, t1568: F, t17443: F, t17446: F, t17451: F, t17499: F, t17547: F, t21089: F, t21207: F, t21242: F, t21247: F, t21306: F, t2886: F, t2930: F, t41826: F, t42111: F, t42113: F, t4433: F, t4471: F, t49099: F, t5775: F, t60775: F, t69003: F, t69005: F, t933: F, t950: F) -> F {
    let t69182 = t21360 * t923;
    let t69218 = F::new(1.0) * t69182 * t933 + t69003 - t69005 - F::cast_from(0.12304822629859687989e5_f64) * t41826 * t21242 * t950 + F::cast_from(0.30762056574649219974e4_f64) * t10756 * t17499 * t4471 + F::cast_from(0.91082604192152556044e5_f64) * t42111 * t21089 * t42113 * t950 + F::cast_from(0.96491876992155210402e2_f64) * t10765 * t21306 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t60775 * t1568 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t17547 * t4433 + F::cast_from(0.10526802520742363173e2_f64) * t14337 * t17443 - F::cast_from(0.70178683471615754484e1_f64) * t14263 * t17446 - F::cast_from(0.31168546390226634765e3_f64) * t49099 * t17451 - F::cast_from(0.14035736694323150897e2_f64) * t10828 * t21247 * t950 + F::cast_from(0.10526802520742363173e2_f64) * t2930 * t5775 * t4471 + F::cast_from(0.6233709278045326953e3_f64) * t10756 * t21207 * t950;
    t69218
}
