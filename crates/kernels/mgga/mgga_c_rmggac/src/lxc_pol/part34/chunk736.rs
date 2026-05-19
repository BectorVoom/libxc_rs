//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 736/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk736<F: Float>(t118: F, t2001: F, t498: F, t699: F, t3203: F, t4616: F, t1347: F, t3208: F, t14584: F, t507: F, t14588: F, t69016: F) -> (F, F, F, F, F, F) {
    let t71154 = t2001 * t118 * t699 * t498;
    let t71158 = t4616 * t3203;
    let t71162 = t1347 * t3208;
    let t71163 = t507 * t14584;
    let t71167 = t507 * t14588;
    let t71196 = F::cast_from(0.16263363996404810741e-4_f64) * t69016;
    (t71154, t71158, t71162, t71163, t71167, t71196)
}
