//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1320/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1320<F: Float>(t44275: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78107: F, t78109: F, t6036: F, t1129: F, t11365: F, t1137: F, t1156: F, t15126: F, t21947: F, t3376: F, t3401: F, t3403: F, t44177: F, t44179: F, t78132: F, t78196: F, t78199: F, t78229: F, t78232: F, t78236: F, t78239: F, t78243: F, t78281: F, t78283: F, t78286: F, t78287: F, t78298: F, t78809: F, t78824: F, t78839: F) -> (F, F) {
    let t78853 = -0.13892666666666666667e0 * t78084 - 0.125034e1 * t78087 + 0.83356e0 * t78090 + 0.375102e1 * t78093 + 0.3529725e1 * t78095 + t44275 + 0.94674375e0 * t78097 + 0.27785333333333333334e0 * t78100 + 0.27545333333333333333e1 * t63361 + 0.1262325e1 * t78103 - 0.705945e1 * t78105 + 0.158837625e2 * t78107 - 0.94674375e0 * t78109 - 0.123954e2 * t78057;
    let t78859 = t6036 * t6036;
    let t78874 = 1.0 * t1129 * (t78809 + t78824 + t78839 + t78853) * t1137 + 0.19964560303604640732e6 * t44177 * t78859 * t44179 + t78132 - t78196 - t78199 - t78229 + t78232 + t78236 - t78239 + t78281 + t78283 - t78286 + t78298 + 0.14035736694323150897e2 * t15126 * t21947 - 0.14035736694323150897e2 * t11365 * t78287 * t1156 - 0.35089341735807877242e1 * t3376 * t78243 * t1156 + 0.51947577317044391277e2 * t3401 * t78243 * t3403;
    (t78859, t78874)
}
