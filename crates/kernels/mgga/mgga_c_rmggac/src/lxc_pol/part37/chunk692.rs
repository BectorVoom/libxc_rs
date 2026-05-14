//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 692/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk692<F: Float>(t13815: F, t2339: F, t7553: F, t13819: F, t8352: F, t2323: F, t2010: F, t2131: F, t8342: F, t2415: F, t7399: F, t68421: F, t68422: F, t73785: F, t15384: F, t34884: F) -> (F, F, F, F, F, F, F) {
    let t74267 = t7553 * t13815 * t2339;
    let t74269 = t13819 * t8352;
    let t74272 = t7553 * t13815 * t2323;
    let t74275 = t2010 * t8342 * t2131;
    let t74278 = t2010 * t2415 * t7399;
    let t74281 = t68421 * t68422 * t73785;
    let t74283 = t34884 * t15384;
    (t74267, t74269, t74272, t74275, t74278, t74281, t74283)
}
