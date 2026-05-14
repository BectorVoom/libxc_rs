//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 915/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk915<F: Float>(t6579: F, t7525: F, t25261: F, t4182: F, t4292: F, t6646: F, t1888: F, t2647: F, t4282: F, t22986: F, t6547: F, t7529: F, t235: F, t25160: F, t4234: F, t6657: F) -> (F, F, F, F, F, F, F) {
    let t25277 = t6579 * t7525;
    let t25281 = t25261 * t4182;
    let t25284 = t6646 * t4292;
    let t25285 = t1888 * t25284;
    let t25287 = t4282 * t2647;
    let t25288 = t6646 * t25287;
    let t25289 = t22986 * t25288;
    let t25293 = t6547 * t7529;
    let t25295 = t235 * t25160;
    let t25297 = t6657 * t4234;
    (t25277, t25281, t25285, t25289, t25293, t25295, t25297)
}
