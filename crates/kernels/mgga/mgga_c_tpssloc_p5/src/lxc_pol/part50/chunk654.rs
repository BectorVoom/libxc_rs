//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 654/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk654<F: Float>(t1338: F, t68: F, t544: F, t1352: F, t5335: F, t1834: F, t5318: F, t553: F, t1332: F, t1336: F, t1381: F, t1383: F, t1814: F, t1838: F, t1840: F, t3777: F, t5230: F, t5234: F, t5334: F, t5336: F, t5339: F, t5341: F, t564: F) -> (F, F, F) {
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t5345 = t5335 * t1352;
    let t5348 = t1338 * t1834;
    let t5349 = t5348 * t1352;
    let t5351 = t553 * t5318;
    let t5353 = t1332 * t1840 - t1336 * t5339 - t1336 * t5341 - t1336 * t5349 - t1381 * t5234 + t1383 * t1814 - t1838 * t3777 + t5230 * t564 + F::cast_from(2.0_f64) * t5334 * t5336 - t5344 * t5345 + t5351 * t544;
    (t5344, t5345, t5353)
}
