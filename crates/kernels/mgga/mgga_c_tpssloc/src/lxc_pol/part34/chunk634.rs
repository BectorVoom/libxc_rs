//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 634/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk634<F: Float>(t2094: F, t532: F, t6884: F, t6899: F, t6914: F, t6921: F, t6934: F, t6948: F, t2086: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t7170 = t532 * t2094;
    let t7174 = 0.38381794893125283518e-1 * t6884;
    let t7176 = 0.82246703342411321825e-2 * t6899;
    let t7181 = 7.0 / 144.0 * t6914;
    let t7183 = 0.28260929265898273597e-2 * t6921;
    let t7185 = 0.67287926823567318088e-4 * t6934;
    let t7189 = 7.0 / 1152.0 * t6948;
    let t7194 = t2086 * t225;
    (t7170, t7174, t7176, t7181, t7183, t7185, t7189, t7194)
}
