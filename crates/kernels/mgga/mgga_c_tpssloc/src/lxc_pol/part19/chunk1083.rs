//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1083/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1083<F: Float>(t236: F, t40041: F, t1336: F, t240: F, t3791: F, t3792: F, t12283: F, t12422: F, t12339: F, t3876: F, t10021: F, t1361: F, t1369: F, t119: F, t12286: F, t12293: F, t12297: F, t12361: F, t1315: F, t1343: F, t210: F, t3733: F, t3783: F, t39622: F, t39892: F, t40012: F, t40019: F, t40022: F, t40025: F, t40026: F, t40035: F, t820: F) -> (F, F, F) {
    let t40042 = t40041 * t236;
    let t40044 = t1336 * t40042 * t240;
    let t40045 = t3791 * t3791;
    let t40046 = t3792 * t3792;
    let t40047 = t40045 * t40046;
    let t40052 = t12283 * t12422;
    let t40054 = t12339 * t3876;
    let t40059 = t1336 * t1361 * t10021;
    let t40060 = t40059 * t1369;
    let t40062 = 7.0 / 36.0 * t40012 - t1315 * t210 * t119 * t39892 / 48.0 + 35.0 / 12.0 * t40019 + 7.0 / 3.0 * t40022 + 5.0 / 4.0 * t40025 * t210 * t119 * t40026 + 3.0 / 16.0 * t3733 * t210 * t119 * t39622 - t40035 * t12293 / 128.0 + t12286 * t12297 / 128.0 + t40044 * t1343 * t820 * t40047 / 128.0 + 35.0 / 96.0 * t40052 + 7.0 / 96.0 * t40054 - t3783 * t12361 / 192.0 + 595.0 / 648.0 * t40060;
    (t40045, t40047, t40062)
}
