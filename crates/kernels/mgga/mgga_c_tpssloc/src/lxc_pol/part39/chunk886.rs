//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 886/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk886<F: Float>(t1036: F, t3089: F, t248: F, t2780: F, t3051: F, t1041: F, t121: F, t3061: F, t2771: F, t1008: F, t349: F, t1011: F) -> (F, F, F, F, F, F) {
    let t10449 = t3089 * t1036;
    let t10454 = t248 * t3051 * t2780;
    let t10455 = t1041 * t10454;
    let t10457 = t121 * t3061;
    let t10459 = t248 * t10457 * t2771;
    let t10460 = t1041 * t10459;
    let t10468 = t1008 * t1008;
    let t10469 = 1.0 / t10468;
    let t10470 = t349 * t10469;
    let t10471 = t1011 * t1011;
    (t10449, t10455, t10460, t10469, t10470, t10471)
}
