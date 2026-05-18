//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 749/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk749<F: Float>(t14617: F, t2106: F, t2145: F, t70502: F, t70510: F, t235: F, t34812: F, t698: F, t36632: F, t14581: F, t7288: F, t14585: F, t7279: F) -> (F, F, F, F, F, F, F) {
    let t72115 = t2145 * t14617 * t2106;
    let t72117 = F::new(0.86737941314158990616e-4) * t70502;
    let t72119 = F::new(0.60975299583150056624e-3) * t70510;
    let t72138 = t235 * t34812 * t698;
    let t72142 = t235 * t36632 * t698;
    let t72145 = t14581 * t7288;
    let t72147 = t14585 * t7279;
    (t72115, t72117, t72119, t72138, t72142, t72145, t72147)
}
