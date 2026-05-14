//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 892/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk892<F: Float>(t114770: F, t22986: F, t28267: F, t28276: F, t31366: F, t6552: F, t23035: F, t31376: F, t5527: F, t6637: F, t121495: F, t1510: F, t6646: F, t121506: F, t1484: F, t114655: F, t121501: F, t126433: F, t126437: F, t126441: F, t31394: F, t33388: F, t4166: F, t5585: F, t5612: F, t5617: F, t812: F) -> (F, F, F) {
    let t127952 = t22986 * t114770 * t28267;
    let t127955 = t6552 * t31366 * t28276;
    let t127959 = t23035 * t6637 * t31376 * t5527;
    let t127963 = t22986 * t6646 * t121495 * t1510;
    let t127967 = t6552 * t6637 * t121506 * t1484;
    let t127979 = t126433 - t126437 + t126441 + 0.49348022005446793095e-1 * t127959 + 0.3289868133696452873e-1 * t127963 - 0.3289868133696452873e-1 * t127967 + 0.16449340668482264365e-1 * t121501 - t812 * t31394 * t5612 + 2.0 * t812 * t114655 * t5585 - t812 * t31394 * t5617 - 2.0 * t4166 * t33388;
    (t127952, t127955, t127979)
}
