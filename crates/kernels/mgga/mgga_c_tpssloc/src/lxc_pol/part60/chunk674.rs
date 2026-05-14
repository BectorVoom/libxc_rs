//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 674/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk674<F: Float>(t23110: F, t7524: F, t23185: F, t1484: F, t252: F, t7510: F, t814: F, t7528: F, t794: F, t6562: F, t1509: F, t1902: F, t6579: F, t7525: F, t6547: F, t7529: F) -> (F, F, F, F, F, F, F) {
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25249 = t252 * t1484;
    let t25255 = t814 * t7510;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    let t25277 = t6579 * t7525;
    let t25293 = t6547 * t7529;
    (t25246, t25249, t25255, t25259, t25261, t25277, t25293)
}
