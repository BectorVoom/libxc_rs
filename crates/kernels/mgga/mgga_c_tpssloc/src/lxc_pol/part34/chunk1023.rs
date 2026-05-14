//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1023/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1023<F: Float>(t23204: F, t28294: F, t6562: F, t28263: F, t28299: F, t81979: F, t28273: F, t6547: F, t28264: F, t28272: F, t794: F, t23164: F, t7479: F, t86893: F, t2105: F, t6470: F) -> (F, F, F, F, F, F, F, F) {
    let t98966 = t6562 * t23204 * t28294;
    let t98983 = t6562 * t23204 * t28263;
    let t98993 = t81979 * t28299;
    let t98995 = t6547 * t28273;
    let t99003 = t6547 * t28264;
    let t99022 = t6562 * t794 * t28272;
    let t99036 = t23164 * t86893 * t7479;
    let t100966 = t6470 * t2105;
    (t98966, t98983, t98993, t98995, t99003, t99022, t99036, t100966)
}
