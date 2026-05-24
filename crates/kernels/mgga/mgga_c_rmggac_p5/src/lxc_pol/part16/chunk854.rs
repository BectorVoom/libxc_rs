//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 854/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk854<F: Float>(t9319: F, t9322: F, t8523: F, t8527: F, t8529: F, t9333: F, t8543: F, t8546: F, t8549: F, t8552: F, t9341: F, t9344: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42425 = F::cast_from(0.35922725105591425692e0_f64) * t9319;
    let t42426 = F::cast_from(0.23948483403727617128e0_f64) * t9322;
    let t42427 = F::cast_from(0.40911992481368012596e-1_f64) * t8523;
    let t42428 = F::cast_from(0.40911992481368012596e-1_f64) * t8527;
    let t42429 = F::cast_from(0.5454932330849068346e-1_f64) * t8529;
    let t42434 = F::cast_from(0.11974241701863808564e0_f64) * t9333;
    let t42435 = F::cast_from(0.11974241701863808564e0_f64) * t8543;
    let t42436 = F::cast_from(0.35922725105591425692e0_f64) * t8546;
    let t42437 = F::cast_from(0.71845450211182851384e0_f64) * t8549;
    let t42438 = F::cast_from(0.17961362552795712846e0_f64) * t8552;
    let t42444 = F::cast_from(0.79828278012425390428e-1_f64) * t9341;
    let t42445 = F::new(0.4726e1) * t9344;
    (t42425, t42426, t42427, t42428, t42429, t42434, t42435, t42436, t42437, t42438, t42444, t42445)
}
