//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 990/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk990<F: Float>(t77906: F, t69166: F, t14451: F, t1587: F, t5259: F, t4669: F, t558: F, t71903: F, t71949: F, t71940: F, t326: F, t650: F, t9565: F) -> (F, F, F, F, F, F, F) {
    let t77907 = F::cast_from(0.44903406381989282115e-1_f64) * t77906;
    let t77908 = F::cast_from(0.79828278012425390427e-1_f64) * t69166;
    let t77910 = t5259 * t14451 * t1587;
    let t77911 = F::cast_from(0.2993560425465952141e-1_f64) * t77910;
    let t77916 = t4669 * t71903 * t558;
    let t77917 = F::cast_from(0.44903406381989282115e-1_f64) * t77916;
    let t77919 = t4669 * t71949 * t558;
    let t77920 = F::cast_from(0.11974241701863808564e0_f64) * t77919;
    let t77921 = F::cast_from(0.39914139006212695213e-1_f64) * t71940;
    let t77929 = t326 * t9565 * t650;
    (t77907, t77908, t77911, t77917, t77920, t77921, t77929)
}
