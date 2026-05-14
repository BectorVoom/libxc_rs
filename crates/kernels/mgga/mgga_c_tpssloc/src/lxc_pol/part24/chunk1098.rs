//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1098/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1098<F: Float>(t23477: F, t23479: F, t6721: F, t6739: F, t6741: F, t1937: F, t23447: F, t23449: F, t23454: F, t23457: F, t23460: F, t23463: F, t23465: F, t23469: F, t23474: F, t350: F, t378: F, t6747: F) -> (F, F, F) {
    let t23480 = t23477 * t23479;
    let t23482 = t6721 * t6739;
    let t23483 = t23482 * t6741;
    let t23486 = -t23447 - 0.16149102437656156342e-2 * t23449 + 0.72670960969452703541e-2 * t23454 * t1937 - 0.16149102437656156342e-2 * t23457 * t1937 + 11.0 / 108.0 * t23460 * t350 - t23463 / 54.0 + t23465 * t378 / 1536.0 - t23469 + 0.20186378047070195428e-3 * t23474 - 0.20186378047070195428e-3 * t23480 - 0.16149102437656156342e-2 * t23483 * t6747;
    (t23482, t23483, t23486)
}
