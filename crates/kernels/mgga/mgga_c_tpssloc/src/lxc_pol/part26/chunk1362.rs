//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1362/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1362<F: Float>(t135: F, t24847: F, t7284: F, t24853: F, t1090: F, t24821: F, t24574: F, t24778: F, t24762: F, t1089: F, t1235: F, t7327: F) -> (F, F, F, F, F) {
    let t86094 = t24847 * t135 * t7284;
    let t86095 = t86094 * t24853;
    let t86102 = t24821 * t1090;
    let t86106 = t24574 * t24778;
    let t86113 = t24574 * t24762;
    let t86116 = t7327 * t1235 * t1089;
    (t86095, t86102, t86106, t86113, t86116)
}
