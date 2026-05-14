//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 730/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk730<F: Float>(t109: F, t4067: F, t656: F, t2327: F, t2328: F, t4041: F, t4044: F, t64: F) -> (F, F) {
    let t110 = 1.0 < t109;
    let t4068 = t656 * t4067;
    let t4072 = piecewise3(t110, 0.0, t2327 + t2328 / 3.0 + t4041 / 3.0 + t64 * t4044 / 4.0 - t64 * t4068 / 8.0);
    (t4068, t4072)
}
