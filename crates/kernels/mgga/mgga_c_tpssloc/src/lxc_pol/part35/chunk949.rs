//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 949/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk949<F: Float>(t21481: F, t68: F, t369: F, t14211: F, t17712: F, t4582: F, t21126: F, t977: F, t21122: F, t2979: F, t10377: F, t10385: F, t10480: F, t10876: F, t10883: F, t14508: F, t14511: F, t17612: F, t17616: F, t21393: F, t21398: F, t21405: F, t3130: F, t378: F, t5875: F, t5880: F, t973: F) -> (F,) {
    let t21482 = t21481 * t68;
    let t21483 = t21482 * t369;
    let t21486 = t17712 * t14211;
    let t21487 = t4582 * t21486;
    let t21490 = t977 * t21126;
    let t21493 = t2979 * t21122;
    let t21498 = t14508 * t5875 / 512.0 + t10480 * t21393 / 512.0 - t10876 * t21398 / 512.0 - t14511 * t5880 / 1024.0 + t10883 * t21405 / 3072.0 + t10377 + t21483 * t378 / 3072.0 + t10385 + t3130 * t21487 / 512.0 - t973 * t21490 / 48.0 + t973 * t21493 / 72.0 + t17612 / 1536.0 + t17616 / 288.0;
    (t21498,)
}
