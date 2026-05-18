//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 775/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk775<F: Float>(t15399: F, t68764: F, t21719: F, t7248: F, t9050: F, t9054: F, t9188: F, t3352: F, t9095: F, t9137: F, t15322: F, t68432: F) -> (F, F, F, F, F, F) {
    let t74015 = t68764 * t15399;
    let t74018 = t21719 * t7248 * t9050;
    let t74021 = t21719 * t9188 * t9054;
    let t74024 = t21719 * t3352 * t9095;
    let t74027 = t21719 * t3352 * t9137;
    let t74033 = t68432 * t15322;
    (t74015, t74018, t74021, t74024, t74027, t74033)
}
