//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1016/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1016<F: Float>(t28342: F, t81979: F, t252: F, t5527: F, t28333: F, t6562: F, t794: F, t22893: F, t23164: F, t28345: F, t28329: F, t23185: F, t28426: F, t81914: F, t28334: F, t6547: F) -> (F, F, F, F, F, F, F) {
    let t98330 = t81979 * t28342;
    let t98336 = t252 * t5527;
    let t98342 = t6562 * t794 * t28333;
    let t98345 = t23164 * t22893 * t28345;
    let t98356 = t23164 * t22893 * t28329;
    let t98363 = t23185 * t81914 * t28426;
    let t98374 = t6547 * t28334;
    (t98330, t98336, t98342, t98345, t98356, t98363, t98374)
}
