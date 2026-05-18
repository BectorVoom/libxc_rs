//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1176/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1176<F: Float>(t25319: F, t5544: F, t6552: F, t6637: F, t23035: F, t5527: F, t1888: F, t21025: F, t22996: F, t22986: F, t25249: F, t5617: F, t6646: F) -> (F, F, F, F) {
    let t105578 = t6552 * t6637 * t25319 * t5544;
    let t105582 = t23035 * t6637 * t25319 * t5527;
    let t105586 = t1888 * t22996 * t21025;
    let t105596 = t22986 * t6646 * t25249 * t5617;
    (t105578, t105582, t105586, t105596)
}
