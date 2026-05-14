//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1177/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1177<F: Float>(t1864: F, t5389: F, t12571: F, t1410: F, t1437: F, t7445: F, t5445: F, t2240: F, t5399: F, t5464: F, t81442: F, t22470: F, t5488: F, t22674: F, t28191: F, t80681: F) -> (F, F, F, F, F, F, F, F) {
    let t96425 = t1864 * t5389;
    let t96443 = t12571 * t1410;
    let t96461 = t7445 * t1437;
    let t96469 = t1864 * t5445;
    let t96473 = t2240 * t5399;
    let t96713 = t81442 * t5464;
    let t96721 = t22470 * t5488;
    let t96848 = t80681 * t22674 * t28191;
    (t96425, t96443, t96461, t96469, t96473, t96713, t96721, t96848)
}
