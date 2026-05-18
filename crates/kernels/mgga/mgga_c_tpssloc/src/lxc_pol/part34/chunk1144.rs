//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1144/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1144<F: Float>(t214: F, t5631: F, t2717: F, t5636: F, t258: F, t5544: F, t28267: F, t81651: F, t82074: F, t5527: F, t857: F, t23204: F, t28298: F, t81640: F) -> (F, F, F, F, F, F) {
    let t98133 = t214 * t5631;
    let t98161 = t2717 * t5636;
    let t98169 = t258 * t5544;
    let t98213 = t81651 * t82074 * t28267;
    let t98224 = t857 * t5527;
    let t98237 = t81640 * t23204 * t28298;
    (t98133, t98161, t98169, t98213, t98224, t98237)
}
