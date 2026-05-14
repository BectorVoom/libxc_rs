//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1153/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1153<F: Float>(t28237: F, t532: F, t2752: F, t28447: F, t23168: F, t28288: F, t214: F, t5631: F, t2717: F, t5636: F, t225: F, t28437: F, t258: F, t5544: F, t28267: F, t81651: F, t82074: F) -> (F, F, F, F, F, F, F, F) {
    let t97817 = t532 * t28237;
    let t98054 = t28447 * t2752;
    let t98117 = t23168 * t28288;
    let t98133 = t214 * t5631;
    let t98161 = t2717 * t5636;
    let t98166 = t28437 * t225;
    let t98169 = t258 * t5544;
    let t98213 = t81651 * t82074 * t28267;
    (t97817, t98054, t98117, t98133, t98161, t98166, t98169, t98213)
}
