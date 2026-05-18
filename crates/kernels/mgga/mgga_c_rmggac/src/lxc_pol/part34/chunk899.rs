//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 899/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk899<F: Float>(t15078: F, t9128: F, t14173: F, t3928: F, t5267: F, t1550: F, t5888: F, t69184: F, t1635: F, t26144: F, t3065: F, t5898: F) -> (F, F, F, F, F) {
    let t76110 = t9128 * t15078;
    let t76113 = t3928 * t14173 * t5267;
    let t76116 = t1550 * t69184 * t5888;
    let t76119 = t26144 * t3065 * t1635;
    let t76122 = t3928 * t14173 * t5898;
    (t76110, t76113, t76116, t76119, t76122)
}
