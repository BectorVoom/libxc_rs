//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 262/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk262<F: Float>(t1174: F, t1195: F, t1198: F, t1203: F, t1213: F, t1218: F, t1224: F, t1227: F, t1232: F, t488: F, t466: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F) -> (F, F, F, F, F, F) {
    let t1235 = t1195 - t1174 * t1198 / 288.0 + t1203 * t488 / 3072.0 + t1213 * t1218 / 3072.0 + t1224 - t1227 * t1232 / 4608.0;
    let t1236 = t466 * t1235;
    let t1238 = t492 * t225;
    let t1239 = t496 * t496;
    let t1240 = 1.0 / t1239;
    let t1241 = t68 * t1240;
    let t1243 = t1011 * t1209;
    (t1235, t1236, t1238, t1239, t1241, t1243)
}
