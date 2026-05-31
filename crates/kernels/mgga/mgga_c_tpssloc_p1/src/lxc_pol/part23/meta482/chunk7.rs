//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1458/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458<F: Float>(t6036: F, t1129: F, t11365: F, t1137: F, t1156: F, t15126: F, t21947: F, t3376: F, t3401: F, t3403: F, t44177: F, t44179: F, t78132: F, t78196: F, t78199: F, t78229: F, t78232: F, t78236: F, t78239: F, t78243: F, t78281: F, t78283: F, t78286: F, t78287: F, t78298: F, t78809: F, t78824: F, t78839: F, t78853: F) -> (F, F) {
    let t78859 = t6036 * t6036;
    let t78874 = F::cast_from(1.0_f64) * t1129 * (t78809 + t78824 + t78839 + t78853) * t1137 + F::cast_from(0.19964560303604640732e6_f64) * t44177 * t78859 * t44179 + t78132 - t78196 - t78199 - t78229 + t78232 + t78236 - t78239 + t78281 + t78283 - t78286 + t78298 + F::cast_from(0.14035736694323150897e2_f64) * t15126 * t21947 - F::cast_from(0.14035736694323150897e2_f64) * t11365 * t78287 * t1156 - F::cast_from(0.35089341735807877242e1_f64) * t3376 * t78243 * t1156 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t78243 * t3403;
    (t78859, t78874)
}
