//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1145/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1145<F: Float>(t5544: F, t857: F, t23164: F, t23204: F, t28276: F, t28342: F, t81979: F, t252: F, t5527: F, t28333: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t98253 = t857 * t5544;
    let t98322 = t23164 * t23204 * t28276;
    let t98330 = t81979 * t28342;
    let t98336 = t252 * t5527;
    let t98342 = t6562 * t794 * t28333;
    (t98253, t98322, t98330, t98336, t98342)
}
