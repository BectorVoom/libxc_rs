//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1188/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188<F: Float>(t3698: F, t3700: F, t1297: F, t1390: F, t16490: F, t193: F, t3719: F, t39852: F, t39854: F, t39856: F, t39858: F, t39892: F, t39932: F, t40222: F, t40224: F, t40226: F, t40228: F, t40230: F, t40232: F, t40234: F, t40603: F, t533: F) -> F {
    let t40608 = t3698 * t3698;
    let t40610 = t3700 * t3700;
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40615 = F::cast_from(36.0_f64) * t193 * t16490 * t3719 - t39852 + t39854 + t39856 - t39858 + F::cast_from(3.0_f64) * t193 * t1297 * t39892 + t193 * t533 * (t39932 + t40603) * t1390 + t40222 + t40224 + t40226 + t40228 - F::cast_from(6.0_f64) * t193 * t533 * t40608 * t40611 - t40230 + t40232 - t40234;
    t40615
}
