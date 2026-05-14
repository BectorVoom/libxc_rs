//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1147/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1147<F: Float>(t2635: F, t41424: F, t2639: F, t9663: F, t232: F, t41367: F, t2630: F, t2681: F, t2701: F, t40926: F, t41395: F, t41397: F, t41399: F, t41404: F, t41410: F, t41415: F, t41417: F, t776: F, t817: F, t819: F, t820: F, t831: F, t843: F, t9516: F, t9613: F) -> (F, F) {
    let t41425 = t41424 * t2635;
    let t41427 = t2639 * t9663;
    let t41429 = t41367 * t232;
    let t41434 = 7.0 / 384.0 * t41395 - 35.0 / 96.0 * t41397 - t41399 * t831 / 768.0 - t9613 * t2681 / 512.0 + 7.0 / 384.0 * t41404 + 7.0 / 1536.0 * t2630 * t819 * t820 * t40926 + t41410 * t2635 / 256.0 + 7.0 / 192.0 * t41415 - 7.0 / 192.0 * t41417 + 5.0 / 192.0 * t843 * t2701 * t820 * t9516 * t776 - 7.0 / 192.0 * t41425 + 7.0 / 1152.0 * t41427 - t817 * t819 * t820 * t41429 / 1024.0;
    (t41429, t41434)
}
