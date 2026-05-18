//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 521/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk521<F: Float>(t14189: F, t3351: F, t2144: F, t7879: F, t1971: F, t13957: F, t875: F, t3154: F, t7720: F, t2160: F, t3061: F, t638: F) -> (F, F, F, F, F, F, F) {
    let t14190 = t3351 * t14189;
    let t14192 = t2144 * t7879;
    let t14193 = t1971 * t14192;
    let t14194 = t3351 * t14193;
    let t14198 = t875 * t13957;
    let t14199 = t1971 * t14198;
    let t14200 = t3351 * t14199;
    let t14202 = t7720 * t3154;
    let t14205 = t638 * t2160 * t3061;
    (t14190, t14193, t14194, t14199, t14200, t14202, t14205)
}
