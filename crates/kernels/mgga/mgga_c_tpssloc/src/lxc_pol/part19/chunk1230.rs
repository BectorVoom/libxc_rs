//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1230/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230<F: Float>(t2402: F, t973: F, t986: F, t3010: F, t698: F, t10327: F, t135: F, t10286: F, t2960: F, t3016: F, t10289: F, t10263: F, t2974: F, t10348: F, t3014: F, t10349: F, t3011: F, t340: F, t343: F, t974: F) -> (F,) {
    let t42903 = t973 * t2402 * t986;
    let t42906 = t973 * t698 * t3010;
    let t42909 = t973 * t135 * t10327;
    let t42911 = t2960 * t10286;
    let t42914 = t973 * t698 * t3016;
    let t42916 = t2960 * t10289;
    let t42918 = t10263 * t2974;
    let t42925 = t973 * t135 * t10348;
    let t42927 = t3014 * t3014;
    let t42933 = -0.12345679012345679012e-2 * t42903 + 0.11111111111111111111e-2 * t42906 - 0.11111111111111111111e-2 * t42909 - 0.59259259259259259257e-2 * t42911 + 0.11111111111111111111e-2 * t42914 + 0.88888888888888888887e-2 * t42916 - 0.32592592592592592592e-1 * t42918 - 0.48888888888888888888e-1 * t10263 * t3011 + 0.88888888888888888888e-2 * t2960 * t10349 - 0.11111111111111111111e-2 * t42925 - 0.83333333333333333332e-3 * t973 * t974 * t340 * t42927 * t343;
    (t42933,)
}
