//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 696/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk696<F: Float>(t1139: F, t4322: F, t1136: F, t1149: F, t1587: F, t3113: F, t4294: F, t4296: F, t4300: F, t473: F, t1589: F, t3154: F, t1151: F, t1153: F, t198: F, t330: F, t4023: F, t4062: F, t4065: F, t4067: F, t4070: F, t4107: F, t4111: F, t4189: F, t4191: F, t4194: F, t4196: F, t4200: F, t4204: F, t4209: F) -> (F, F, F, F) {
    let t4323 = t1139 * t4322;
    let t4325 = 2.0 * t1136 * t4300 - t1136 * t4323 - t1149 * t4296 - t1587 * t3113 + t4294 * t473;
    let t4329 = t1589 * t3154;
    let t4332 = t1153 * t198 * t330 * t4325 - t1151 * t4023 * t4329 - t4062 + t4065 + t4067 - t4070 + t4107 + t4111 + t4189 + t4191 - t4194 - t4196 + t4200 - t4204 - t4209;
    (t4323, t4325, t4329, t4332)
}
