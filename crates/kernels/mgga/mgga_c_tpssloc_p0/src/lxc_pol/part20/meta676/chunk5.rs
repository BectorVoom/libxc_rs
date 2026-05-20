//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2555/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2555<F: Float>(t11356: F, t11366: F, t11434: F, t1148: F, t1156: F, t15133: F, t3334: F, t3371: F, t3378: F, t436: F, t44211: F, t4802: F, t4858: F, t51107: F, t51669: F, t51677: F, t51680: F, t51725: F, t51727: F, t51730: F, t51736: F, t51738: F, t51741: F, t51744: F, t51765: F, t51785: F) -> F {
    let t51789 = -F::new(6.0) * t44211 * t4802 - t51669 + F::cast_from(0.17544670867903938621e1_f64) * t11356 * t4858 + F::cast_from(0.17544670867903938621e1_f64) * t3371 * t15133 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t51107 * t1156 - F::cast_from(0.35089341735807877242e1_f64) * t51677 * t3378 - F::cast_from(0.10389515463408878255e3_f64) * t51680 * t11366 - F::cast_from(0.19751673498613801407e-1_f64) * t51725 - F::cast_from(0.31168546390226634766e3_f64) * t51727 * t11434 - F::new(6.0) * t51730 * t3334 - t51736 - t51738 - t51741 - t51744 - F::new(0.310907e-1) * (t51765 + t51785) * t436;
    t51789
}
