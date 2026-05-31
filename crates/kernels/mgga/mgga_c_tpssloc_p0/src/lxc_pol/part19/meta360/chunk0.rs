//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1308/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1308<F: Float>(t42340: F, t42341: F, t3034: F, t368: F, t3128: F, t3040: F, t10882: F, t42333: F, t1015: F, t1041: F, t10914: F, t13969: F) -> (F, F, F, F, F, F, F, F) {
    let t42342 = t42340 * t42341;
    let t42343 = t3034 * t3034;
    let t42344 = F::cast_from(1.0_f64) / t42343;
    let t42345 = t368 * t42344;
    let t42347 = t42342 * t3128 * t42345;
    let t42348 = t3040 * t3040;
    let t42354 = t42333 * t10882;
    let t42358 = t42342 * t1015 * t42345;
    let t42369 = t1041 * t13969 * t10914;
    (t42342, t42344, t42345, t42347, t42348, t42354, t42358, t42369)
}
