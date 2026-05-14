//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 879/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk879<F: Float>(t121495: F, t25038: F, t25248: F, t776: F, t33429: F, t6547: F, t7841: F, t857: F, t22986: F, t23270: F, t31338: F, t86873: F, t33422: F, t114601: F, t1527: F, t1888: F) -> (F, F, F, F, F, F) {
    let t121612 = t25038 * t25248 * t121495 * t776;
    let t121629 = t6547 * t33429;
    let t121634 = t857 * t7841;
    let t121637 = t22986 * t23270 * t121634 * t776;
    let t121648 = t22986 * t86873 * t31338;
    let t121660 = t6547 * t33422;
    let t121689 = t1888 * t23270 * t114601 * t1527;
    (t121612, t121629, t121637, t121648, t121660, t121689)
}
