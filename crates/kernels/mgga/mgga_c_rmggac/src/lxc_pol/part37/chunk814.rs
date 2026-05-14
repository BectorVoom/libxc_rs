//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 814/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk814<F: Float>(t14710: F, t2868: F, t739: F, t7799: F, t9530: F, t74953: F, t74957: F, t3351: F, t498: F, t515: F, t7248: F, t9523: F, t9188: F, t9527: F, t71210: F, t74961: F) -> (F, F, F, F, F, F, F, F) {
    let t77282 = t2868 * t14710;
    let t77283 = 0.2993560425465952141e-1 * t77282;
    let t77286 = 0.11974241701863808564e0 * t739 * t9530 * t7799;
    let t77287 = 0.2553875993597870364e-4 * t74953;
    let t77288 = 0.7661627980793611092e-4 * t74957;
    let t77292 = t3351 * t7248 * t515 * t9523 * t498;
    let t77293 = 0.12769379967989351819e-4 * t77292;
    let t77296 = t3351 * t9188 * t515 * t9527;
    let t77297 = 0.25538759935978703638e-4 * t77296;
    let t77299 = 0.36021158228745895953e-3 * t71210;
    let t77300 = 0.20455996240684006298e-1 * t74961;
    (t77283, t77286, t77287, t77288, t77293, t77297, t77299, t77300)
}
