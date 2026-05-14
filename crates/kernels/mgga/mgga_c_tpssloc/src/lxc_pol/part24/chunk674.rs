//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 674/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk674<F: Float>(t131: F, t2570: F, t205: F, t242: F, t2628: F, t812: F, t244: F, t67: F, t246: F, t157: F, t2658: F, t228: F, t68: F, t2627: F, t226: F, t814: F) -> (F, F, F, F, F, F, F, F) {
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4177 = t2628 * t242;
    let t4178 = t812 * t4177;
    let t4179 = t244 * t67;
    let t4180 = t4179 * t246;
    let t4194 = t2658 * t157;
    let t4225 = t228 * t68;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4290 = t68 * t814;
    (t4127, t4178, t4179, t4180, t4194, t4225, t4281, t4290)
}
