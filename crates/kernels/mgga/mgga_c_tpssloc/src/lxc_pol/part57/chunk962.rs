//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 962/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk962<F: Float>(t118727: F, t118738: F, t1888: F, t232: F, t6646: F, t98524: F, t98494: F, t118744: F, t118747: F, t1484: F, t6552: F, t6637: F) -> (F, F, F, F, F, F) {
    let t126453 = F::cast_from(0.3289868133696452873e-1_f64) * t118727;
    let t126456 = F::cast_from(0.76763589786250567036e-1_f64) * t118738;
    let t126472 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t6646 * t98524 * t232;
    let t126476 = F::cast_from(0.16449340668482264365e-1_f64) * t1888 * t6646 * t98494 * t232;
    let t126477 = F::cast_from(0.15352717957250113407e0_f64) * t118744;
    let t126481 = F::cast_from(0.6579736267392905746e-1_f64) * t6552 * t6637 * t118747 * t1484;
    (t126453, t126456, t126472, t126476, t126477, t126481)
}
