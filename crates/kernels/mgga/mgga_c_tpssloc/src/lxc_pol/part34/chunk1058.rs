//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1058/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1058<F: Float>(t22633: F, t22897: F, t26421: F, t6388: F, t1825: F, t26331: F, t6976: F, t97011: F, t6420: F, t1992: F, t550: F, t75026: F, t1985: F, t1998: F, t20601: F, t214: F) -> (F, F, F, F, F) {
    let t107381 = t22633 * t22897 * t26421 * t6388;
    let t107385 = t26331 * t6976 * t97011 * t1825;
    let t107389 = t22633 * t6976 * t26421 * t6420;
    let t107397 = t1992 * t6976 * t75026 * t550;
    let t107402 = t1985 * t214 * t1998 * t20601;
    (t107381, t107385, t107389, t107397, t107402)
}
