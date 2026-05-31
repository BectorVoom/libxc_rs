//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 424/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk424<F: Float>(t1395: F, t3: F, t576: F, t112: F, t577: F, t671: F, t71: F, t79: F, t193: F, t202: F, t154: F, t204: F) -> (F, F, F, F, F, F, F) {
    let t1396 = t3 * t1395;
    let t1398 = t3 * t576;
    let t1401 = t576 * t112;
    let t1404 = F::cast_from(0.45e1_f64) * t1395 * t577 + F::cast_from(0.135e2_f64) * t1401 * t671;
    let t1864 = t71 * t79;
    let t1877 = t193 * t202;
    let t1878 = t204 * t154;
    (t1396, t1398, t1401, t1404, t1864, t1877, t1878)
}
