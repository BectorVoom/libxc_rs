//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 641/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk641<F: Float>(t14148: F, t240: F, t356: F, t4738: F, t7351: F, t14107: F, t6477: F, t14207: F, t2604: F, t1966: F, t1968: F, t68889: F, t14226: F, t14020: F, t68536: F, t14019: F, t14027: F) -> (F, F, F, F, F, F, F) {
    let t70524 = t14148 * t7351 * t356 * t240 * t4738;
    let t70526 = t6477 * t14107;
    let t70545 = t2604 * t14207;
    let t70548 = t1966 * t68889 * t1968;
    let t70549 = t70548 * t14226;
    let t70554 = t14020 * t68536;
    let t70556 = t14019 * t70554 * t14027;
    (t70524, t70526, t70545, t70548, t70549, t70554, t70556)
}
