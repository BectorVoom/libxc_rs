//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1103/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1103<F: Float>(t21061: F, t225: F, t21036: F, t20856: F, t252: F, t1519: F, t5584: F, t20852: F, t5611: F, t1509: F, t5631: F, t21064: F, t21682: F, t21689: F, t21669: F, t21684: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    let t67350 = t252 * t20856;
    let t67358 = t1519 * t5584;
    let t67392 = t252 * t20852;
    let t67405 = t1519 * t5611;
    let t68025 = t5611 * t1509;
    let t68217 = t5631 * t1509;
    let t68322 = t21064 * t225;
    let t69871 = t21682 * t225;
    let t70978 = t21689 * t225;
    let t70980 = t21669 * t225;
    let t70987 = t21684 * t225;
    (t67339, t67344, t67350, t67358, t67392, t67405, t68025, t68217, t68322, t69871, t70978, t70980, t70987)
}
