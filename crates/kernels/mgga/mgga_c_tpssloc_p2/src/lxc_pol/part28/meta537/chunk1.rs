//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1797/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1797<F: Float>(t22988: F, t23110: F, t81651: F, t22893: F, t23154: F, t23164: F, t234: F, t2710: F, t23176: F, t23185: F, t131: F, t2587: F, t81142: F) -> (F, F, F, F, F) {
    let t81653 = t81651 * t23110 * t22988;
    let t81656 = t23164 * t22893 * t23154;
    let t81658 = t234 * t2710;
    let t81670 = t23185 * t23110 * t23176;
    let t81686 = t81142 * t2587 * t131;
    (t81653, t81656, t81658, t81670, t81686)
}
