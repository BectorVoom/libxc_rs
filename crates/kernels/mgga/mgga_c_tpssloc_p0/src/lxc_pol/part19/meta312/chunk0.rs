//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1112/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1112<F: Float>(t1294: F, t39344: F, t9810: F, t9844: F, t39321: F, t12458: F, t12461: F, t677: F, t9713: F, t3684: F, t181: F, t2558: F, t686: F) -> (F, F, F, F, F, F, F) {
    let t39346 = F::cast_from(0.46785788981077169656e1_f64) * t1294 * t39344;
    let t39347 = t9810 * t9844;
    let t39349 = F::cast_from(0.19263893255070628432e1_f64) * t39321 * t39347;
    let t39350 = t12458 * t12461;
    let t39354 = t677 * t9713;
    let t39356 = F::cast_from(0.21687162600603479684e-1_f64) * t3684 * t39354;
    let t39358 = t686 * t2558 * t181;
    (t39346, t39347, t39349, t39350, t39354, t39356, t39358)
}
