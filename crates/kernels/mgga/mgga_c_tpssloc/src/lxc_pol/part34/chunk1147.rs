//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1147/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1147<F: Float>(t23110: F, t23185: F, t28422: F, t23168: F, t28346: F, t28338: F, t81591: F, t252: F, t5544: F, t28337: F, t81651: F, t28423: F, t6579: F) -> (F, F, F, F, F, F) {
    let t98399 = t23185 * t23110 * t28422;
    let t98416 = t23168 * t28346;
    let t98420 = t81591 * t28338;
    let t98422 = t252 * t5544;
    let t98446 = t81651 * t23110 * t28337;
    let t98488 = t6579 * t28423;
    (t98399, t98416, t98420, t98422, t98446, t98488)
}
