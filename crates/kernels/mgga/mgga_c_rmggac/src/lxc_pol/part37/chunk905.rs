//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 905/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk905<F: Float>(t69092: F, t69095: F, t73321: F, t73322: F, t75010: F, t77384: F, t77388: F, t77392: F, t77397: F, t77399: F, t77402: F, t77405: F, t77406: F, t77418: F, t77421: F, t77424: F) -> (F,) {
    let t80204 = t77384 - t77388 + t77392 + t77397 - t77399 + t77402 - t77405 + t77406 + 0.58171619854173713844e-5 * t75010 - t73321 + t73322 + t69092 + t69095 + t77418 + t77421 - t77424;
    (t80204,)
}
