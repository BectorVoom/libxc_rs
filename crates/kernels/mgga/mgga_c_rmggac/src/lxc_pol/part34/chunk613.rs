//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 613/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk613<F: Float>(t34750: F, t637: F, t26007: F, t271: F, t69097: F, t71: F, t14084: F, t35244: F, t35228: F, t3075: F, t68800: F, t2079: F, t262: F, t664: F, t830: F, t2123: F, t265: F) -> (F, F, F, F, F, F, F) {
    let t69101 = t34750 * t637;
    let t69102 = t26007 * t69097 * t271 * t71 * t69101;
    let t69104 = t14084 * t35244;
    let t69105 = 0.25650144397517585626e-6 * t69104;
    let t69106 = t14084 * t35228;
    let t69107 = 0.25650144397517585626e-6 * t69106;
    let t69108 = t3075 * t68800;
    let t69114 = t2079 * t262 * t830 * t664;
    let t69130 = t2079 * t262 * t265 * t2123;
    (t69101, t69102, t69105, t69107, t69108, t69114, t69130)
}
