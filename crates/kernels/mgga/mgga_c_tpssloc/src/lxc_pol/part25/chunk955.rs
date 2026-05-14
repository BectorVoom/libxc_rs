//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 955/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk955<F: Float>(t218: F, t24234: F, t7084: F, t798: F, t23013: F, t23031: F, t2684: F, t7101: F, t2047: F, t2627: F, t2633: F, t22990: F, t23000: F, t23002: F, t23006: F, t23022: F, t23026: F, t23028: F, t23038: F, t2617: F, t7102: F, t812: F) -> (F, F, F, F, F, F) {
    let t24235 = t218 * t24234;
    let t24237 = t798 * t7084;
    let t24246 = 0.12793931631041761173e0 * t23013;
    let t24250 = 0.52089578783527170489e-1 * t23031;
    let t24251 = t7101 * t2684;
    let t24255 = t2627 * t2047;
    let t24256 = t24255 * t2633;
    let t24260 = 0.6579736267392905746e-1 * t22990 + 0.3289868133696452873e-1 * t23000 + 0.76763589786250567036e-1 * t23002 - 0.16449340668482264365e-1 * t23006 + t24246 + 0.16449340668482264365e-1 * t23022 - 0.16449340668482264365e-1 * t23026 - 0.76763589786250567036e-1 * t23028 + t24250 - t812 * t24251 - 2.0 * t2617 * t7102 + 2.0 * t812 * t24256 + 0.9869604401089358619e-1 * t23038;
    (t24235, t24237, t24251, t24255, t24256, t24260)
}
