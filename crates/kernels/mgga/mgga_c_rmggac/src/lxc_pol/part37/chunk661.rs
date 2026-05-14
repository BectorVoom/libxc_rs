//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 661/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk661<F: Float>(t72037: F, t14506: F, t2085: F, t69518: F, t69521: F, t14584: F, t504: F, t14617: F, t2106: F, t2145: F, t70502: F, t70510: F, t235: F, t34812: F, t698: F, t36632: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t72038 = 0.14635184302277988245e0 * t72037;
    let t72062 = t14506 * t2085;
    let t72087 = 0.34547904762044099522e0 * t69518;
    let t72088 = 0.50557909407869413937e0 * t69521;
    let t72109 = t504 * t14584;
    let t72115 = t2145 * t14617 * t2106;
    let t72117 = 0.86737941314158990616e-4 * t70502;
    let t72119 = 0.60975299583150056624e-3 * t70510;
    let t72138 = t235 * t34812 * t698;
    let t72142 = t235 * t36632 * t698;
    (t72038, t72062, t72087, t72088, t72109, t72115, t72117, t72119, t72138, t72142)
}
