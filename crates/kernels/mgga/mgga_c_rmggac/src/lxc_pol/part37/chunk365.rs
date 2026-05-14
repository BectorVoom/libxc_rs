//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 365/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk365<F: Float>(t131: F, t270: F, t31: F, t1179: F, t214: F, t132: F, t1338: F, t668: F, t934: F, t4179: F, t6: F, t211: F, t483: F, t1976: F, t5542: F) -> (F, F, F, F, F, F, F, F) {
    let t7352 = t131 * t270;
    let t7353 = t7352 * t31;
    let t7363 = t1179 * t214;
    let t7385 = t132 * t1338;
    let t7399 = t934 * t668;
    let t7417 = t6 * t4179;
    let t7427 = t211 * t483;
    let t7472 = t1976 * t5542;
    (t7352, t7353, t7363, t7385, t7399, t7417, t7427, t7472)
}
