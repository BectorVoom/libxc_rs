//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 612/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk612<F: Float>(t875: F, t9551: F, t1971: F, t3351: F, t2338: F, t702: F, t638: F, t639: F, t2474: F, t640: F, t3219: F, t8571: F) -> (F, F, F, F, F, F, F) {
    let t15488 = t875 * t9551;
    let t15489 = t1971 * t15488;
    let t15490 = t3351 * t15489;
    let t15491 = F::cast_from(0.85129199786595678796e-5_f64) * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    let t15495 = F::cast_from(0.15243824895787514157e-3_f64) * t15494;
    let t15496 = t640 * t2474;
    let t15498 = t638 * t639 * t15496;
    let t15499 = F::cast_from(0.15243824895787514157e-3_f64) * t15498;
    let t15500 = t8571 * t3219;
    (t15489, t15491, t15492, t15495, t15496, t15499, t15500)
}
