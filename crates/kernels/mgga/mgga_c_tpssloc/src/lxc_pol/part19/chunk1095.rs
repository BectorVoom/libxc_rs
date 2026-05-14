//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1095/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1095<F: Float>(t40270: F, t550: F, t1336: F, t1339: F, t2691: F, t3809: F, t12267: F, t3865: F, t1369: F, t1362: F, t40118: F, t12344: F, t3777: F, t12361: F, t3866: F, t12336: F, t12379: F, t12392: F, t12397: F, t12404: F, t12429: F, t1341: F, t1343: F, t1363: F, t1367: F, t3778: F, t3858: F, t3876: F, t39892: F, t40206: F, t820: F) -> (F, F) {
    let t40271 = t40270 * t550;
    let t40281 = t1336 * t1339 * t2691;
    let t40282 = t40281 * t3809;
    let t40284 = t12267 * t3865;
    let t40285 = t40284 * t1369;
    let t40287 = t40118 * t1362;
    let t40292 = t3777 * t12344;
    let t40293 = t40292 * t1369;
    let t40295 = t3866 * t12361;
    let t40303 = 7.0 / 1152.0 * t40206 - t3778 * t12392 / 768.0 - t1341 * t1343 * t820 * t40271 / 3072.0 - t3778 * t12379 / 768.0 - t12397 * t3858 / 512.0 + 119.0 / 288.0 * t40282 + 7.0 / 96.0 * t40285 - t40287 * t1369 / 192.0 - t12336 * t3876 / 128.0 - 119.0 / 288.0 * t40293 + 7.0 / 288.0 * t40295 - t1363 * t1367 * t820 * t39892 / 768.0 + t12429 * t12404 / 64.0;
    (t40271, t40303)
}
