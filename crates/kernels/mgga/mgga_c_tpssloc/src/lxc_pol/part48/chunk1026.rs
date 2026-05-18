//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1026/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1026<F: Float>(t115275: F, t115277: F, t115279: F, t115283: F, t115666: F, t115669: F, t115672: F, t115674: F, t115676: F, t115678: F, t117533: F, t2040: F, t2323: F, t23929: F, t31832: F, t32350: F, t3652: F, t672: F, t7156: F, t7218: F, t7264: F, t7266: F, t8329: F, t85428: F, t8829: F) -> F {
    let t117604 = -F::new(4.0) * t117533 * t672 - F::new(2.0) * t2040 * t85428 - F::new(4.0) * t2323 * t32350 - F::new(4.0) * t23929 * t7266 + F::new(2.0) * t31832 * t7218 - t3652 * t8829 - F::new(2.0) * t7156 * t7264 - t115275 - t115277 - t115279 + t115283 + t115666 - t115669 - t115672 - t115674 - t115676 + t115678 - t8329;
    t117604
}
