//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 395/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk395<F: Float>(t2392: F, t262: F, t2079: F, t2376: F, t305: F, t2379: F, t326: F, t118: F, t2292: F, t2367: F, t338: F) -> (F, F, F, F, F) {
    let t2393 = t262 * t2392;
    let t2394 = t2079 * t2393;
    let t2396 = t305 * t2376;
    let t2398 = t326 * t2379;
    let t2400 = t118 * t2292;
    let t2402 = t338 * t2367;
    (t2394, t2396, t2398, t2400, t2402)
}
