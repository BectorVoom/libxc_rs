//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1041/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1041<F: Float>(t30816: F, t6733: F, t6743: F, t6746: F, t1003: F, t8387: F, t1014: F, t368: F, t1017: F, t1012: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t30817 = t6733 * t30816;
    let t30820 = t6743 * sigma0;
    let t30821 = t30820 * t6746;
    let t30824 = t1003 * t8387;
    let t30827 = t1014 * t368;
    let t30828 = t30827 * t1017;
    let t30829 = t1012 * t30828;
    (t30817, t30820, t30821, t30824, t30827, t30828, t30829)
}
