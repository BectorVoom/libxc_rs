//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1271/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1271<F: Float>(t10098: F, t1888: F, t6646: F, t229: F, t268: F, t6559: F, t22988: F, t23110: F, t22893: F, t23154: F, t23164: F, t234: F, t2710: F) -> (F, F, F, F, F) {
    let t81648 = t1888 * t6646 * t10098;
    let t81651 = t6559 * t229 * t268;
    let t81653 = t81651 * t23110 * t22988;
    let t81656 = t23164 * t22893 * t23154;
    let t81658 = t234 * t2710;
    (t81648, t81651, t81653, t81656, t81658)
}
