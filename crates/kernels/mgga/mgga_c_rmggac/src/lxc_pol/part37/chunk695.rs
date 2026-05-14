//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 695/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk695<F: Float>(t68386: F, t7248: F, t8667: F, t8830: F, t9188: F, t3352: F, t8835: F, t8842: F, t1971: F, t2144: F, t3351: F, t40940: F, t15266: F, t16043: F, t41063: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t74324 = t68386 * t7248 * t8667;
    let t74327 = t68386 * t9188 * t8830;
    let t74330 = t68386 * t3352 * t8835;
    let t74333 = t68386 * t3352 * t8842;
    let t74337 = t3351 * t1971 * t2144 * t40940;
    let t74339 = t16043 * t15266;
    let t74345 = t3351 * t1971 * t875 * t41063;
    (t74324, t74327, t74330, t74333, t74337, t74339, t74345)
}
