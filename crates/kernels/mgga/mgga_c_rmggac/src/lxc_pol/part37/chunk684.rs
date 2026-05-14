//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 684/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk684<F: Float>(t16503: F, t16504: F, t665: F, t9151: F, t3369: F, t9157: F, t2024: F, t34976: F, t9163: F, t1971: F, t2144: F, t3351: F, t41122: F, t3148: F, t3151: F, t38350: F) -> (F, F, F, F, F) {
    let t74088 = t16503 * t16504 * t665 * t9151;
    let t74092 = t16503 * t3369 * t665 * t9157;
    let t74096 = t16503 * t34976 * t2024 * t9163;
    let t74102 = t3351 * t1971 * t2144 * t41122;
    let t74105 = t38350 * t3148 * t3151;
    (t74088, t74092, t74096, t74102, t74105)
}
