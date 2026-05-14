//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 811/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk811<F: Float>(t74333: F, t74337: F, t74339: F, t74345: F, t74354: F, t74356: F, t74368: F, t74371: F, t74374: F, t3351: F, t3352: F, t875: F, t9577: F, t68540: F, t68543: F, t68550: F, t74378: F, t74381: F, t74387: F, t74390: F) -> (F,) {
    let t76972 = 0.15961724959986689775e-4 * t74333;
    let t76973 = 0.2553875993597870364e-4 * t74337;
    let t76974 = 0.1702583995731913576e-4 * t74339;
    let t76975 = 0.1702583995731913576e-4 * t74345;
    let t76976 = 0.1702583995731913576e-4 * t74354;
    let t76977 = 0.85129199786595678799e-5 * t74356;
    let t76978 = 0.85129199786595678799e-5 * t74368;
    let t76979 = 0.15961724959986689775e-4 * t74371;
    let t76980 = 0.1276937996798935182e-4 * t74374;
    let t76985 = t3351 * t3352 * t875 * t9577;
    let t76986 = 0.25538759935978703638e-4 * t76985;
    let t76989 = -t76972 + t76973 + t76974 + t68540 - t68543 + t76975 + t76976 - t76977 + t68550 + t76978 - t76979 - t76980 - 0.17519306092901367187e-5 * t74378 - 0.39418438709028076171e-5 * t74381 + t76986 + 0.70077224371605468752e-6 * t74387 - 0.70077224371605468752e-6 * t74390;
    (t76989,)
}
