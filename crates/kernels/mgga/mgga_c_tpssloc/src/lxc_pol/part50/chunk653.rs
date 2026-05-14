//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 653/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk653<F: Float>(t363: F, t6743: F, t1022: F, t360: F, t68: F, t1004: F, t1941: F, t1014: F, sigma0: F) -> (F, F, F, F, F) {
    let t6744 = t6743 * t363;
    let t6746 = t1022 * t68 * t360;
    let t6747 = t6744 * t6746;
    let t6750 = t1004 * t1941;
    let t6753 = t1014 * sigma0;
    (t6744, t6746, t6747, t6750, t6753)
}
