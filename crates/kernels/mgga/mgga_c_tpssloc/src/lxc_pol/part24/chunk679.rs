//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 679/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk679<F: Float>(t671: F, t88: F, t193: F, t531: F, t533: F, t131: F, t3732: F, t205: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t546: F, t68: F) -> (F, F, F, F, F, F, F, F) {
    let t5113 = t88 * t671;
    let t5126 = t193 * t531;
    let t5160 = t193 * t533;
    let t5194 = t3732 * t131;
    let t5195 = t205 * t5194;
    let t5245 = t3788 * t242;
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    let t5278 = t546 * t68;
    (t5113, t5126, t5160, t5195, t5246, t5247, t5248, t5278)
}
