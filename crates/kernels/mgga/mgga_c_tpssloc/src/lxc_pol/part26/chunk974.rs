//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 974/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk974<F: Float>(t12465: F, t12474: F, t12476: F, t12490: F, t3652: F, t671: F, t1266: F, t2363: F, t113: F, t11968: F, t11972: F, t1271: F, t1393: F, t2312: F, t2314: F, t2320: F, t2323: F, t2364: F, t3660: F, t3929: F, t4034: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F, t9347: F, t9348: F, t9351: F, t9419: F) -> (F, F, F, F) {
    let t12492 = t12465 + t12474 + t12476 + t12490;
    let t12504 = t3652 * t671;
    let t12507 = t1266 * t2363;
    let t12512 = -t113 * t11968 - 2.0 * t11972 * t652 + t12492 * t513 - 6.0 * t12504 * t652 - 6.0 * t12507 * t652 - 3.0 * t1266 * t2312 - 6.0 * t1266 * t2320 + 3.0 * t1271 * t3929 + 3.0 * t1393 * t3660 - 12.0 * t2314 * t2323 - 6.0 * t2314 * t2364 - 6.0 * t2364 * t4034 - 3.0 * t3652 * t650 - t510 * t9347 - 6.0 * t510 * t9351 + t574 * t9419 - 6.0 * t672 * t9348;
    (t12492, t12504, t12507, t12512)
}
