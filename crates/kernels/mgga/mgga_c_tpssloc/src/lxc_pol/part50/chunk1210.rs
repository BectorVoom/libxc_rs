//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1210/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1210<F: Float>(t25749: F, t8375: F, t6688: F, t7593: F, t118971: F, t1921: F, t1052: F, t1055: F, t113314: F, t113318: F, t113619: F, t119201: F, t119232: F, t119366: F, t119393: F, t1955: F, t1956: F, t23346: F, t23394: F, t25419: F, t25731: F, t25738: F, t25801: F, t25826: F, t30788: F, t30854: F, t3169: F, t3174: F, t32909: F, t32917: F, t32961: F, t32993: F, t388: F, t6687: F, t6691: F, t6704: F, t7565: F, t88145: F, t986: F, t990: F) -> F {
    let t119407 = t8375 * t25749;
    let t119412 = t6688 * t7593;
    let t119420 = t1921 * t118971;
    let t119440 = F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t32993 - t1052 * t1055 * (t119201 + t119232 + t119366 + t119393) - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t30854 * t25826 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t30854 * t25738 - F::cast_from(0.54831135561607547883e-2_f64) * t113314 - F::new(2.0) * t88145 * t1956 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t119407 - F::cast_from(0.14621636149762012769e-1_f64) * t113318 + F::cast_from(0.54831135561607547883e-2_f64) * t6687 * t119412 * t6691 + F::new(4.0) * t3169 * t32917 + t990 * t32961 * t388 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t119420 - F::new(6.0) * t3169 * t32909 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t6704 * t23394 * t25419 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t113619 * t7565 + F::cast_from(0.54831135561607547883e-2_f64) * t6687 * t30788 * t25801 + F::new(4.0) * t1052 * t3174 * t1955 * t25731;
    t119440
}
