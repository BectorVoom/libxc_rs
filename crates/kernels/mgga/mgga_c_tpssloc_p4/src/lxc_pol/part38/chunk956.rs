//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 956/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk956<F: Float>(t1041: F, t10459: F, t1008: F, t349: F, t1011: F) -> (F, F, F, F) {
    let t10460 = t1041 * t10459;
    let t10468 = t1008 * t1008;
    let t10469 = F::new(1.0) / t10468;
    let t10470 = t349 * t10469;
    let t10471 = t1011 * t1011;
    (t10460, t10469, t10470, t10471)
}
