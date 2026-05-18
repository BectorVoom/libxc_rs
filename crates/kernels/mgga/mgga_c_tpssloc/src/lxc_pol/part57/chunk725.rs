//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 725/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk725<F: Float>(t6579: F, t7525: F, t6547: F, t7529: F, t23168: F, t7521: F, t22893: F, t7520: F, t23164: F, t1519: F, t234: F, t23204: F, t7479: F) -> (F, F, F, F, F, F) {
    let t25277 = t6579 * t7525;
    let t25293 = t6547 * t7529;
    let t25310 = t23168 * t7521;
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25345 = t23204 * t7479;
    (t25277, t25293, t25310, t25317, t25319, t25345)
}
