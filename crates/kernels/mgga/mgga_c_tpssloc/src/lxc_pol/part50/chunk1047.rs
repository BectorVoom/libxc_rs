//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1047/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1047<F: Float>(t22986: F, t32814: F, t82159: F, t32815: F, t81591: F, t112899: F, t1888: F, t25045: F, t23270: F, t30633: F, t98960: F, t25038: F, t25040: F, t32862: F, t112667: F, t112673: F) -> (F, F, F, F, F, F, F, F) {
    let t118479 = 0.3289868133696452873e-1 * t22986 * t82159 * t32814;
    let t118480 = t81591 * t32815;
    let t118481 = 0.76763589786250567037e-1 * t118480;
    let t118484 = 0.3289868133696452873e-1 * t1888 * t112899 * t25045;
    let t118488 = 0.6579736267392905746e-1 * t22986 * t23270 * t30633 * t98960;
    let t118491 = 0.9869604401089358619e-1 * t25038 * t112899 * t25040;
    let t118498 = 0.3289868133696452873e-1 * t1888 * t82159 * t32862;
    let t118499 = 0.38381794893125283518e-1 * t112667;
    let t118500 = 0.38381794893125283518e-1 * t112673;
    (t118479, t118481, t118484, t118488, t118491, t118498, t118499, t118500)
}
