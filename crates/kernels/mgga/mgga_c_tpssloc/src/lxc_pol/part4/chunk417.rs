//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 417/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk417<F: Float>(t1352: F, t1380: F, t1372: F, t553: F, t1332: F, t1336: F, t544: F, t564: F, t1378: F, t1324: F, t1373: F, t1375: F, t568: F) -> (F, F, F, F, F) {
    let t1381 = t1380 * t1352;
    let t1383 = t553 * t1372;
    let t1385 = t1332 * t564 - t1336 * t1381 + t1383 * t544;
    let t1386 = t1378 * t1385;
    let t1388 = t1324 * t568 + t1373 * t568 - t1375 * t1386;
    (t1381, t1383, t1385, t1386, t1388)
}
