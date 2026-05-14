//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 748/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk748<F: Float>(t172: F, t6320: F, t763: F, t1824: F, t1834: F, t562: F, t6387: F, t118: F, t6330: F, t794: F, t12202: F, t6347: F, t3739: F, t12211: F, t6353: F, t3726: F, t6358: F) -> (F, F, F, F, F, F, F) {
    let t19681 = t6320 * t172;
    let t19682 = t19681 * t763;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19775 = t118 * t794 * t6347;
    let t19776 = t3739 * t19775;
    let t19779 = t12211 * t6353;
    let t19791 = t3726 * t6358;
    (t19682, t19739, t19743, t19768, t19776, t19779, t19791)
}
