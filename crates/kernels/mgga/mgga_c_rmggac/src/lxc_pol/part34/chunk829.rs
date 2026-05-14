//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 829/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk829<F: Float>(t74953: F, t74957: F, t3351: F, t498: F, t515: F, t7248: F, t9523: F, t9188: F, t9527: F, t71207: F, t74927: F, t74929: F, t74930: F, t74932: F, t77265: F, t77271: F, t77275: F, t77279: F, t77280: F, t77281: F, t77283: F, t77286: F) -> (F,) {
    let t77287 = 0.2553875993597870364e-4 * t74953;
    let t77288 = 0.7661627980793611092e-4 * t74957;
    let t77292 = t3351 * t7248 * t515 * t9523 * t498;
    let t77293 = 0.12769379967989351819e-4 * t77292;
    let t77296 = t3351 * t9188 * t515 * t9527;
    let t77297 = 0.25538759935978703638e-4 * t77296;
    let t77298 = t77265 - t74927 + t74929 + 0.93188427318671584245e-2 * t74930 - 0.15531404553111930708e-1 * t74932 - t71207 - t77271 + t77275 + t77279 + t77280 - t77281 - t77283 + t77286 - t77287 + t77288 + t77293 - t77297;
    (t77298,)
}
