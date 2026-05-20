//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1242/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1242<F: Float>(t1307: F, t1842: F, t1527: F, t776: F, t671: F, t7982: F, t2169: F, t214: F, t6624: F, t30657: F, t6547: F, t30671: F) -> (F, F, F, F, F, F, F) {
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    let t112667 = t6547 * t30657;
    let t112673 = t6547 * t30671;
    (t97721, t98960, t104977, t105108, t112660, t112667, t112673)
}
