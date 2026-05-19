//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 426/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk426<F: Float>(t259: F, t379: F, t1474: F, t1464: F, t366: F, t220: F, t368: F, t983: F, t985: F, t981: F, t373: F, t978: F, t1402: F, t1413: F, t1427: F, t1453: F, t1455: F, t1459: F, t198: F, t330: F, t995: F) -> (F, F, F, F, F, F) {
    let t380 = t259 < t379;
    let t1475 = param_beta * t1474;
    let t1477 = t366 * t1464;
    let t1482 = t1474 * t220 * t368 + t1477 * t983 * t985;
    let t1483 = t981 * t1482;
    let t1485 = t1475 * t373 - t1483 * t978;
    let t1490 = piecewise3::<F>(t380, t1485 * t198 * t330 * t995 - t1413 + t1427 + t1453 + t1455 - t1459, t1402);
    (t1475, t1477, t1482, t1483, t1485, t1490)
}
