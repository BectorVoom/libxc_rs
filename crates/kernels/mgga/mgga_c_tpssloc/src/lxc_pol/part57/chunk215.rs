//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 215/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk215<F: Float>(t122: F, t374: F, t486: F, t485: F, t372: F, t483: F, t479: F, t471: F, t404: F, t415: F, t61: F, t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1222 = t374 * t122 * t486;
    let t1224 = t485 * t1222 / 4608.0;
    let t1225 = t483 * t372;
    let t1226 = t479 * t1225;
    let t1227 = t471 * t1226;
    let t1229 = 1.0 / t415 / t404;
    let t1230 = t61 * t1229;
    let t1238 = t492 * t225;
    let t1239 = t496 * t496;
    let t1240 = 1.0 / t1239;
    let t1241 = t68 * t1240;
    let t1243 = t1011 * t1209;
    (t1222, t1224, t1226, t1227, t1229, t1230, t1238, t1239, t1241, t1243)
}
