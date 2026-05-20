//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1304/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1304<F: Float>(t32815: F, t81591: F, t112899: F, t1888: F, t25045: F, t22986: F, t23270: F, t30633: F, t98960: F, t25038: F, t25040: F, t32862: F, t82159: F) -> (F, F, F, F, F) {
    let t118480 = t81591 * t32815;
    let t118481 = F::cast_from(0.76763589786250567037e-1_f64) * t118480;
    let t118484 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t112899 * t25045;
    let t118488 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t23270 * t30633 * t98960;
    let t118491 = F::cast_from(0.9869604401089358619e-1_f64) * t25038 * t112899 * t25040;
    let t118498 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t82159 * t32862;
    (t118481, t118484, t118488, t118491, t118498)
}
