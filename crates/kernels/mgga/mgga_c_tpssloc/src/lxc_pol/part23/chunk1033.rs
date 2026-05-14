//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1033/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1033<F: Float>(t268: F, t521: F, t9799: F, t9847: F, t677: F, t9494: F, t3684: F, t2505: F, t2527: F, t1294: F, t2368: F, t747: F, t9711: F, t9810: F, t9844: F, t9713: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39321 = t521 * t268;
    let t39322 = t9799 * t9847;
    let t39324 = 0.1301229756036208781e0 * t39321 * t39322;
    let t39325 = t677 * t9494;
    let t39327 = 0.38025319932552508021e2 * t3684 * t39325;
    let t39336 = t2527 * t2505;
    let t39338 = 0.21053605041484726346e2 * t1294 * t39336;
    let t39344 = t2368 * t9711 * t747;
    let t39346 = 0.46785788981077169656e1 * t1294 * t39344;
    let t39347 = t9810 * t9844;
    let t39349 = 0.19263893255070628432e1 * t39321 * t39347;
    let t39354 = t677 * t9713;
    (t39322, t39324, t39325, t39327, t39336, t39338, t39344, t39346, t39347, t39349, t39354)
}
