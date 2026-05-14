//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 446/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk446<F: Float>(t1519: F, t218: F, t1510: F, t860: F, t235: F, t1499: F, t226: F, t255: F, t812: F, t858: F, t1493: F, t259: F, t855: F, t1464: F, t1473: F, t1476: F, t1484: F, t193: F, t202: F, t680: F, t705: F, t752: F, t760: F, t765: F, t766: F, t870: F) -> (F, F, F, F, F, F, F) {
    let t1520 = t218 * t1519;
    let t1523 = t860 * t1510;
    let t1525 = t235 * t1519;
    let t1527 = t1499 * t255 - t1523 * t812 + t1525 * t226;
    let t1528 = t858 * t1527;
    let t1530 = t1493 * t259 + t1520 * t259 - t1528 * t855;
    let t1534 = t1530 * t193 * t202 * t870 + 3.0 * t1484 * t193 * t766 + t1464 + t1473 + t1476 + t680 + t705 + t752 - t760 - t765;
    (t1520, t1523, t1525, t1527, t1528, t1530, t1534)
}
