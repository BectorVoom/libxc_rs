//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 433/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk433<F: Float>(t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t1297: F, t1307: F, t1388: F, t1390: F, t193: F, t533: F, t680: F, t705: F, t113: F, t1266: F, t1271: F, t510: F, t513: F, t574: F, t650: F, t652: F, t672: F) -> (F, F) {
    let t1393 = t1388 * t1390 * t193 * t533 + 3.0 * t1297 * t1307 * t193 + t1274 - t1276 + t1286 + t1288 + t1290 - t1293 - t1296 + t680 + t705;
    let t1395 = -t113 * t1266 + t1271 * t574 + t1393 * t513 - t510 * t650 - 2.0 * t652 * t672;
    (t1393, t1395)
}
