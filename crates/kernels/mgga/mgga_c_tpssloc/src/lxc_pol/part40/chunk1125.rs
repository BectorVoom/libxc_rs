//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1125/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1125<F: Float>(t1345: F, t1348: F, t1819: F, t1821: F, t19702: F, t19708: F, t19716: F, t19719: F, t19725: F, t19728: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F) -> (F,) {
    let t19731 = -12.0 * t1345 * t6408 + 3.0 * t1345 * t6411 + 3.0 * t1348 * t6404 + 6.0 * t1819 * t5283 + 6.0 * t1821 * t5272 - t19702 * t548 - 24.0 * t19708 * t5280 + 60.0 * t19716 * t5278 - 24.0 * t19719 * t5278 - 12.0 * t19725 * t5278 + 3.0 * t19728 * t546;
    (t19731,)
}
