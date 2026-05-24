//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1047/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1047<F: Float>(t1982: F, t2314: F, t35512: F, t118: F, t128: F, t2001: F, t5738: F, t675: F, t2289: F, t7921: F, t333: F, t3351: F, t511: F, t9210: F, t9211: F) -> (F, F, F, F) {
    let t41767 = t2314 * t35512 * t1982;
    let t41772 = t675 * t2001 * t118 * t128 * t5738;
    let t41774 = t7921 * t2289;
    let t41779 = t3351 * t9210 * t511 * t9211 * t333;
    (t41767, t41772, t41774, t41779)
}
