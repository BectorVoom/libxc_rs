//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 332/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk332<F: Float>(t1090: F, t1230: F, t248: F, t1174: F, t1195: F, t1198: F, t1203: F, t1213: F, t1218: F, t1224: F, t1227: F, t488: F) -> (F, F) {
    let t1232 = t248 * t1230 * t1090;
    let t1235 = t1195 - t1174 * t1198 / F::new(288.0) + t1203 * t488 / F::new(3072.0) + t1213 * t1218 / F::new(3072.0) + t1224 - t1227 * t1232 / F::new(4608.0);
    (t1232, t1235)
}
