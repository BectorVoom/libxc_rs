//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1349/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349<F: Float>(t113: F, t12492: F, t12507: F, t1266: F, t1271: F, t12734: F, t1393: F, t2314: F, t2320: F, t2364: F, t3652: F, t3660: F, t39223: F, t39231: F, t39235: F, t3929: F, t39332: F, t39385: F, t39480: F, t39524: F, t39586: F, t39626: F, t39847: F, t40615: F, t43657: F, t45402: F, t510: F, t513: F, t672: F, t89: F, t9347: F, t9351: F, t9419: F) -> (F,) {
    let t45405 = -t39223 * t510 - 24.0 * t9351 * t1266 - 12.0 * t2320 * t3652 - 4.0 * t9347 * t1266 - 6.0 * t89 * t39231 * t510 - 8.0 * t39235 * t672 - 24.0 * t12734 * t2364 - 24.0 * t2314 * t12507 + 6.0 * t3660 * t3929 + 4.0 * t9419 * t1393 + t513 * (t39332 + t39385 + t39480 + t39524 + t39586 + t39626 + t39847 + t40615) + 4.0 * t1271 * t12492 - t113 * (t43657 + t45402);
    (t45405,)
}
