//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1167/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1167<F: Float>(t1888: F, t22996: F, t2632: F, t81672: F, t23110: F, t23180: F, t23185: F, t206: F, t22723: F, t268: F, t23186: F, t1902: F, t2678: F, t23163: F, t23165: F, t10046: F, t1880: F, t1894: F, t214: F) -> (F, F, F, F, F, F, F) {
    let t82025 = t1888 * t22996 * t81672 * t2632;
    let t82028 = t23185 * t23110 * t23180;
    let t82031 = t22723 * t206 * t268;
    let t82032 = t82031 * t23186;
    let t82034 = t1902 * t2678;
    let t82038 = t22723 * t23163;
    let t82039 = t82038 * t23165;
    let t82043 = t1880 * t214 * t1894 * t10046;
    (t82025, t82028, t82032, t82034, t82038, t82039, t82043)
}
