//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1166/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1166<F: Float>(t22986: F, t23270: F, t30633: F, t98960: F, t112899: F, t25038: F, t25040: F, t1888: F, t32862: F, t82159: F, t112667: F, t112673: F, t25170: F, t112678: F, t112680: F, t112686: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118488 = 0.6579736267392905746e-1 * t22986 * t23270 * t30633 * t98960;
    let t118491 = 0.9869604401089358619e-1 * t25038 * t112899 * t25040;
    let t118498 = 0.3289868133696452873e-1 * t1888 * t82159 * t32862;
    let t118499 = 0.38381794893125283518e-1 * t112667;
    let t118500 = 0.38381794893125283518e-1 * t112673;
    let t118503 = 0.9869604401089358619e-1 * t1888 * t23270 * t25170;
    let t118506 = 0.82246703342411321825e-2 * t112678;
    let t118518 = 0.76763589786250567036e-1 * t112680;
    let t118523 = 0.76763589786250567036e-1 * t112686;
    (t118488, t118491, t118498, t118499, t118500, t118503, t118506, t118518, t118523)
}
