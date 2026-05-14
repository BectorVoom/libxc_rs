//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 33/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk33<F: Float>(t53: F, t60: F, t31: F, t36: F, rho0: F, rho1: F, tau0: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t83 = pow_1_3(rho0);
    let t84 = t83 * t83;
    let t86 = 1.0 / t84 / rho0;
    let t87 = tau0 * t86;
    let t88 = t53 / 2.0;
    let t89 = pow_1_3(t88);
    let t90 = t89 * t89;
    let t91 = t90 * t88;
    let t94 = pow_1_3(rho1);
    let t95 = t94 * t94;
    let t97 = 1.0 / t95 / rho1;
    let t98 = tau1 * t97;
    let t99 = t60 / 2.0;
    let t100 = pow_1_3(t99);
    let t101 = t100 * t100;
    let t102 = t101 * t99;
    let t107 = 2.0 * t87 * t91 + 2.0 * t98 * t102 - t31 * t36 / 4.0;
    (t84, t87, t89, t90, t91, t95, t98, t100, t101, t102, t107)
}
