//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 550/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk550<F: Float>(t15489: F, t3351: F, t2338: F, t702: F, t638: F, t639: F, t2474: F, t640: F, t3219: F, t8571: F, t618: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t15490 = t3351 * t15489;
    let t15491 = 0.85129199786595678796e-5 * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    let t15495 = 0.15243824895787514157e-3 * t15494;
    let t15496 = t640 * t2474;
    let t15498 = t638 * t639 * t15496;
    let t15499 = 0.15243824895787514157e-3 * t15498;
    let t15500 = t8571 * t3219;
    let t15501 = 0.42564599893297839398e-5 * t15500;
    let t15502 = t698 * t618;
    (t15491, t15492, t15495, t15496, t15499, t15501, t15502)
}
