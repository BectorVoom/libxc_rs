//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 324/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk324<F: Float>(t1214: F, t1216: F, t248: F, t122: F, t374: F, t486: F, t485: F, t372: F, t483: F, t479: F, t471: F) -> (F, F, F, F, F) {
    let t1218 = t248 * t1214 * t1216;
    let t1222 = t374 * t122 * t486;
    let t1224 = t485 * t1222 / 4608.0;
    let t1225 = t483 * t372;
    let t1226 = t479 * t1225;
    let t1227 = t471 * t1226;
    (t1218, t1222, t1224, t1226, t1227)
}
