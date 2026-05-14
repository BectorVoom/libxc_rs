//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1353/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1353<F: Float>(t109: F, t45509: F, t11968: F, t11972: F, t12504: F, t12507: F, t1266: F, t1268: F, t12734: F, t2312: F, t2314: F, t2323: F, t2363: F, t2364: F, t3652: F, t39223: F, t39231: F, t39235: F, t4034: F, t45408: F, t510: F, t5113: F, t574: F, t650: F, t652: F, t671: F, t88: F, t9348: F, t9416: F) -> (F, F) {
    let t110 = 1.0 < t109;
    let t45510 = piecewise3(t110, 0.0, t45509);
    let t45545 = (2.0 * t1268 * t45510 + 24.0 * t12734 * t2363 + 8.0 * t2314 * t9416 + 12.0 * t2363 * t9348 + 6.0 * t39231 * t88 + 8.0 * t39235 * t671 + 8.0 * t5113 * t9416 + t39223 + 12.0 * t45408) * t574 - 12.0 * t9348 * t2364 - 8.0 * t2314 * t11972 - 24.0 * t4034 * t12507 - 8.0 * t4034 * t11972 - 8.0 * t652 * t1266 * t9416 - 24.0 * t9348 * t2323 - 8.0 * t652 * t11968 * t671 - 24.0 * t2314 * t12504 - 12.0 * t652 * t3652 * t2363 - 6.0 * t2312 * t3652 - 12.0 * t45408 * t510 - 4.0 * t650 * t11968 - 2.0 * t652 * t510 * t45510;
    (t45510, t45545)
}
