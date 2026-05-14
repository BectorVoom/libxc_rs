//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1119/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1119<F: Float>(t1527: F, t776: F, t671: F, t7982: F, t2169: F, t214: F, t6624: F, t30657: F, t6547: F, t30671: F, t23030: F, t30660: F, t23204: F, t30656: F, t6562: F, t30624: F, t81591: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98960 = t1527 * t776;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    let t112667 = t6547 * t30657;
    let t112673 = t6547 * t30671;
    let t112676 = 0.52089578783527170489e-1 * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    let t112680 = t81591 * t30624;
    (t98960, t104977, t105108, t112660, t112667, t112673, t112676, t112678, t112680)
}
