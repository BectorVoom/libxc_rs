//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1033/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1033<F: Float>(t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t7740: F, t794: F, t6897: F, t552: F, t6604: F, t7696: F, t12461: F, t2094: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    let t26446 = t6604 * t552;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26558 = t2094 * t12461;
    (t26426, t26427, t26429, t26436, t26437, t26446, t26474, t26475, t26558)
}
