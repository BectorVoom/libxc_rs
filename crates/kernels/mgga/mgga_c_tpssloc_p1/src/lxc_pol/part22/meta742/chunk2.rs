//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2454/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2454<F: Float>(t21472: F, t2970: F, t973: F, t13822: F, t21452: F, t21468: F, t42972: F, t21453: F, t21469: F, t21473: F, t2960: F, t48293: F, t48321: F, t61383: F, t61387: F, t61391: F, t61394: F, t61397: F, t61405: F, t61408: F, t61422: F, t61427: F) -> F {
    let t69796 = t973 * t2970 * t21472;
    let t69801 = t973 * t13822 * t21452;
    let t69806 = t973 * t42972 * t21468;
    let t69817 = -F::cast_from(0.44444444444444444443e-2_f64) * t2960 * t21473 + F::cast_from(0.55555555555555555553e-3_f64) * t69796 + F::cast_from(0.66666666666666666664e-2_f64) * t2960 * t21453 - F::cast_from(0.8333333333333333333e-3_f64) * t69801 - F::cast_from(0.23045267489711934156e-2_f64) * t2960 * t21469 + F::cast_from(0.28806584362139917695e-3_f64) * t69806 - F::cast_from(0.98765432098765432096e-3_f64) * t61383 + F::cast_from(0.16666666666666666666e-2_f64) * t61387 - F::cast_from(0.11111111111111111111e-2_f64) * t61391 - F::cast_from(0.22222222222222222221e-2_f64) * t61394 + F::cast_from(0.55555555555555555554e-3_f64) * t61397 - F::cast_from(0.74074074074074074072e-3_f64) * t61405 + F::cast_from(0.37037037037037037036e-3_f64) * t61408 - F::cast_from(0.16666666666666666666e-2_f64) * t61422 - F::cast_from(0.83333333333333333331e-3_f64) * t61427 + t48293 - t48321;
    t69817
}
