//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 628/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk628<F: Float>(t14042: F, t69836: F, t14115: F, t68454: F, t14147: F, t14151: F, t7348: F, t1295: F, t131: F, t14148: F, t25987: F, t7351: F, t14059: F, t14371: F, t69452: F, t739: F) -> (F, F, F, F, F, F) {
    let t69837 = t69836 * t14042;
    let t69839 = t68454 * t14115;
    let t69860 = t14147 * t7348 * t14151;
    let t69865 = t14148 * t7351 * t131 * t1295 * t25987;
    let t69870 = t14371 * t14059;
    let t69894 = t739 * t69452;
    (t69837, t69839, t69860, t69865, t69870, t69894)
}
