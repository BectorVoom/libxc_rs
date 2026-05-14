//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 550/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk550<F: Float>(t143: F, t655: F, t130: F, t675: F, t676: F) -> (F, F, F, F, F) {
    let t2286 = t655 * t143;
    let t2287 = 1.0 / t2286;
    let t2288 = t130 * t2287;
    let t2289 = t675 * t675;
    let t2290 = t2289 * t676;
    let t2292 = 2.0 * t2288 * t2290;
    (t2287, t2288, t2289, t2290, t2292)
}
