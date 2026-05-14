//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1089/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1089<F: Float>(t1992: F, t28135: F, t6347: F, t6968: F, t6637: F, t6888: F, t6330: F, t22685: F, t1799: F, t26395: F, t6415: F, t6987: F, t1336: F, t1814: F, t2013: F, t22693: F, t26381: F, t26427: F, t27082: F, t27088: F, t28132: F, t6378: F, t7747: F) -> (F, F, F, F, F, F, F, F) {
    let t28136 = t1992 * t28135;
    let t28138 = t6968 * t6347;
    let t28139 = t6637 * t28138;
    let t28140 = t6888 * t28139;
    let t28142 = t6968 * t6330;
    let t28143 = t6637 * t28142;
    let t28144 = t22685 * t28143;
    let t28148 = t26395 * t1799;
    let t28149 = t6637 * t28148;
    let t28150 = t6888 * t28149;
    let t28152 = t6987 * t6415;
    let t28155 = 0.76763589786250567036e-1 * t26381 - t22693 + t6378 * t2013 + t27082 + 0.3289868133696452873e-1 * t28132 + 0.16449340668482264365e-1 * t28136 + t27088 - 0.16449340668482264365e-1 * t28140 + 0.49348022005446793095e-1 * t28144 + 2.0 * t1814 * t7747 - 0.3289868133696452873e-1 * t28150 - t1336 * t28152 + 0.82246703342411321824e-2 * t26427;
    (t28138, t28139, t28142, t28143, t28148, t28149, t28152, t28155)
}
