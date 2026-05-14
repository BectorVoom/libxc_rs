//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 713/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk713<F: Float>(t1937: F, t23447: F, t23449: F, t23454: F, t23457: F, t23460: F, t23463: F, t23465: F, t23469: F, t23474: F, t23480: F, t23483: F, t350: F, t378: F, t6747: F, t344: F, t6729: F) -> (F, F) {
    let t23486 = -t23447 - 0.16149102437656156342e-2 * t23449 + 0.72670960969452703541e-2 * t23454 * t1937 - 0.16149102437656156342e-2 * t23457 * t1937 + 11.0 / 108.0 * t23460 * t350 - t23463 / 54.0 + t23465 * t378 / 1536.0 - t23469 + 0.20186378047070195428e-3 * t23474 - 0.20186378047070195428e-3 * t23480 - 0.16149102437656156342e-2 * t23483 * t6747;
    let t23488 = t6729 * t344;
    (t23486, t23488)
}
