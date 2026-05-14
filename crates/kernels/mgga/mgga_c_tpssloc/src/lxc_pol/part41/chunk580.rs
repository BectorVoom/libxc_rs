//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 580/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk580<F: Float>(t1053: F, t386: F, t68: F, t1057: F, t3112: F, t3032: F, t3127: F, t3031: F) -> (F, F, F, F) {
    let t3173 = 1.0 / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3180 = t3112 * t1057;
    let t3185 = t3032 * t3127;
    let t3186 = t3031 * t3185;
    (t3174, t3180, t3185, t3186)
}
