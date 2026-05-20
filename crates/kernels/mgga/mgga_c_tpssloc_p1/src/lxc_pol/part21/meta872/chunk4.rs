//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3216/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216<F: Float>(t113: F, t12545: F, t1271: F, t12816: F, t1393: F, t1458: F, t15857: F, t16503: F, t1778: F, t1849: F, t19289: F, t19537: F, t20098: F, t20136: F, t2312: F, t2314: F, t3652: F, t3660: F, t3929: F, t4028: F, t4034: F, t510: F, t513: F, t5450: F, t55568: F, t55927: F, t56110: F, t56124: F, t56148: F, t56161: F, t56174: F, t56192: F, t56212: F, t56294: F, t56364: F, t56370: F, t56389: F, t56408: F, t57801: F, t57810: F, t57815: F, t57822: F, t6287: F, t6295: F, t63261: F, t6468: F, t650: F, t652: F, t66921: F) -> F {
    let t66935 = -F::new(8.0) * t4034 * t20136 - F::new(4.0) * t652 * t15857 * t1458 - F::new(8.0) * t2314 * t20136 - F::new(8.0) * t4028 * t12545 - F::new(2.0) * t652 * t510 * t55568 + F::new(2.0) * t12816 * t1849 + F::new(2.0) * t19537 * t1393 + t513 * (t56110 + t56124 + t56148 + t56161 + t56174 + t56192 + t56212 + t56294 + t56364 + t56370 + t56389 + t56408 + t57801 + t57810 + t57815 + t57822) - t113 * (t63261 + t66921) - t2312 * t6287 - F::new(2.0) * t650 * t19289 - t55927 * t510 - t5450 * t3652 + t6295 * t3929 + F::new(2.0) * t1271 * t20098 + t3660 * t6468 + F::new(2.0) * t1778 * t16503;
    t66935
}
