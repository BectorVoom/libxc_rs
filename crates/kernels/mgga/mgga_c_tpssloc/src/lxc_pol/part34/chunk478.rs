//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 478/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk478<F: Float>(t1043: F, t2775: F, t2770: F, t3061: F, t135: F, t1606: F, t973: F, t1036: F, t1612: F, t1616: F, t248: F, t3101: F, t1020: F, t1009: F, t1603: F, t1011: F) -> (F, F, F, F, F, F, F, F) {
    let t4583 = t1043 * t2775;
    let t4588 = t3061 * t2770;
    let t4603 = t135 * t1606;
    let t4604 = t973 * t4603;
    let t4625 = t1612 * t1036;
    let t4630 = t248 * t3101 * t1616;
    let t4631 = t1020 * t4630;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    (t4583, t4588, t4604, t4625, t4630, t4631, t4639, t4640)
}
