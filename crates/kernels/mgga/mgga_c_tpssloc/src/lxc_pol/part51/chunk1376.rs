//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1376/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1376<F: Float>(t31526: F, t7685: F, t33483: F, t868: F, t1914: F, t26756: F, t584: F, t86730: F, t193: F, t8574: F, t33476: F, t4119: F) -> (F, F, F, F, F, F) {
    let t121254 = t7685 * t31526;
    let t121258 = t33483 * t868;
    let t121264 = t26756 * t86730 * t584 * t1914;
    let t121271 = t193 * t8574;
    let t121275 = t33476 * t868;
    let t121279 = t1914 * t4119;
    (t121254, t121258, t121264, t121271, t121275, t121279)
}
