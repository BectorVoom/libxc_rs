//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 966/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk966<F: Float>(t1406: F, t2239: F, t584: F, t9212: F, t111: F, t4025: F) -> (F, F, F, F) {
    let t12571 = t1406 * t2239;
    let t12603 = F::new(2.0) * t584;
    let t12604 = F::new(6.0) * t9212;
    let t12725 = t4025 * t111;
    (t12571, t12603, t12604, t12725)
}
