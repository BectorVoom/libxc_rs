//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 907/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk907<F: Float>(t2231: F, t2338: F, t638: F, t639: F, t2164: F, t2474: F, t15612: F, t275: F, t76521: F, t76524: F, t76527: F, t72145: F, t72147: F, t70582: F, t2211: F, t41122: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t78566 = t638 * t639 * t2338 * t2231;
    let t78567 = 0.15243824895787514157e-3 * t78566;
    let t78570 = t638 * t639 * t2164 * t2474;
    let t78571 = 0.15243824895787514157e-3 * t78570;
    let t78572 = t275 * t15612;
    let t78574 = 0.30487649791575028312e-3 * t76521;
    let t78575 = 0.16263363996404810741e-4 * t76524;
    let t78576 = 0.16263363996404810741e-4 * t76527;
    let t78577 = 0.27274661654245341729e-1 * t72145;
    let t78578 = 0.36366215538993788972e-1 * t72147;
    let t78582 = 0.86737941314158990619e-4 * t70582;
    let t78585 = 0.11974241701863808564e0 * t884 * t2211 * t41122;
    (t78567, t78571, t78572, t78574, t78575, t78576, t78577, t78578, t78582, t78585)
}
