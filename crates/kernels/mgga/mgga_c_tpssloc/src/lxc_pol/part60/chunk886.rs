//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 886/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk886<F: Float>(t1484: F, t31376: F, t6637: F, t6552: F, t232: F, t26656: F, t6646: F, t1888: F, t1894: F, t7823: F, t214: F, t1880: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33375 = t31376 * t1484;
    let t33376 = t6637 * t33375;
    let t33377 = t6552 * t33376;
    let t33379 = t26656 * t232;
    let t33380 = t6646 * t33379;
    let t33381 = t1888 * t33380;
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    (t33375, t33376, t33377, t33379, t33380, t33381, t33383, t33384, t33385)
}
