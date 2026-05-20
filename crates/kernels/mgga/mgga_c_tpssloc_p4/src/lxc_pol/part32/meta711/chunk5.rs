//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2230/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2230<F: Float>(t23168: F, t28346: F, t28338: F, t81591: F, t252: F, t5544: F, t22986: F, t6646: F, t829: F, t16759: F, t1888: F, t17030: F, t2647: F) -> (F, F, F, F, F, F) {
    let t98416 = t23168 * t28346;
    let t98420 = t81591 * t28338;
    let t98422 = t252 * t5544;
    let t98425 = t22986 * t6646 * t98422 * t829;
    let t98428 = t1888 * t6646 * t16759;
    let t98432 = t22986 * t6646 * t17030 * t2647;
    (t98416, t98420, t98422, t98425, t98428, t98432)
}
