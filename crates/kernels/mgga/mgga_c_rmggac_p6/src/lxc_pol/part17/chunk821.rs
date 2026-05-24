//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 821/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk821<F: Float>(t1986: F, t2318: F, t326: F, t333: F, t551: F, t7817: F, t1550: F, t2289: F, t7939: F, t2323: F, t638: F, t7184: F) -> (F, F, F, F, F) {
    let t40323 = t1986 * t326 * t2318 * t333;
    let t40331 = t7817 * t551;
    let t40332 = t1550 * t40331;
    let t40339 = t7939 * t2289;
    let t40343 = t638 * t7184 * t2323;
    (t40323, t40331, t40332, t40339, t40343)
}
