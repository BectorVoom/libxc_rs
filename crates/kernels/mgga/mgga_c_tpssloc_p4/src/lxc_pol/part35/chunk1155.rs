//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1155/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1155<F: Float>(t7510: F, t814: F, t7528: F, t794: F, t6562: F, t1509: F, t1902: F, t6579: F, t7525: F, t6547: F, t7529: F, t23168: F, t7521: F) -> (F, F, F, F, F, F, F) {
    let t25255 = t814 * t7510;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    let t25277 = t6579 * t7525;
    let t25293 = t6547 * t7529;
    let t25310 = t23168 * t7521;
    (t25255, t25258, t25259, t25261, t25277, t25293, t25310)
}
