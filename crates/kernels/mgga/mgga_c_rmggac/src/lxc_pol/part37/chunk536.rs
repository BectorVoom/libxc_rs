//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 536/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk536<F: Float>(t3069: F, t8659: F, t3077: F, t8365: F, t128: F, t589: F, t118: F, t14011: F, t14047: F, t2319: F, t14052: F, t13862: F, t2282: F, t3120: F, t553: F, t2412: F, t3154: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15197 = t8659 * t3069;
    let t15199 = t8365 * t3077;
    let t15203 = t128 * t589;
    let t15204 = t118 * t15203;
    let t15205 = t14011 * t15204;
    let t15206 = t14047 * t15205;
    let t15208 = t14011 * t2319;
    let t15209 = t14052 * t15208;
    let t15211 = t13862 * t2282;
    let t15212 = t3120 * t15211;
    let t15214 = t14011 * t553;
    let t15215 = t3120 * t15214;
    let t15218 = t2412 * t3154;
    (t15197, t15199, t15204, t15205, t15206, t15208, t15209, t15211, t15212, t15214, t15215, t15218)
}
