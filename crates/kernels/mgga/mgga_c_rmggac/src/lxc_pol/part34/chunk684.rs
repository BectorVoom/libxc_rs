//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 684/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk684<F: Float>(t14236: F, t14243: F, t2067: F, t55986: F, t15371: F, t69568: F, t68524: F, t14063: F, t3151: F, t8450: F, t15363: F, t3140: F, t13872: F, t13876: F, t13880: F, t13884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74159 = t14236 * t14243 * t2067 * t55986;
    let t74161 = t69568 * t15371;
    let t74163 = t68524 * t15371;
    let t74166 = t8450 * t14063 * t3151;
    let t74168 = t68524 * t15363;
    let t74170 = t8450 * t3140;
    let t74171 = t74170 * t13872;
    let t74173 = t74170 * t13876;
    let t74175 = t74170 * t13880;
    let t74177 = t74170 * t13884;
    (t74159, t74161, t74163, t74166, t74168, t74171, t74173, t74175, t74177)
}
