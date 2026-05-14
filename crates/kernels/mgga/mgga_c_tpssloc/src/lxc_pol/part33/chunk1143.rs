//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1143/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1143<F: Float>(t1862: F, t2240: F, t5392: F, t1409: F, t605: F, t111: F, t27992: F, t5464: F, t81442: F, t22470: F, t5488: F, t22674: F, t28191: F, t80681: F, t28206: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t96547 = t2240 * t5392 * t1862;
    let t96551 = t605 * t1409 * t1862;
    let t96686 = t27992 * t111;
    let t96713 = t81442 * t5464;
    let t96721 = t22470 * t5488;
    let t96848 = t80681 * t22674 * t28191;
    let t96868 = t6883 * t28206;
    (t96547, t96551, t96686, t96713, t96721, t96848, t96868)
}
