//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1317/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1317<F: Float>(t23168: F, t32819: F, t234: F, t7510: F, t6552: F, t6637: F, t776: F, t112951: F, t1484: F, t1888: F, t232: F, t6646: F, t87567: F) -> (F, F, F, F) {
    let t118744 = t23168 * t32819;
    let t118745 = F::cast_from(0.76763589786250567037e-1_f64) * t118744;
    let t118747 = t234 * t7510;
    let t118751 = F::cast_from(0.3289868133696452873e-1_f64) * t6552 * t6637 * t118747 * t776;
    let t118756 = F::cast_from(0.3289868133696452873e-1_f64) * t6552 * t6637 * t112951 * t1484;
    let t118760 = F::cast_from(0.16449340668482264365e-1_f64) * t1888 * t6646 * t87567 * t232;
    (t118745, t118751, t118756, t118760)
}
