//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 412/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk412<F: Float>(t45: F, t57: F, t1289: F, t190: F, t681: F, t78: F, t81: F, t150: F, t162: F, t187: F, t741: F, t745: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t1342 = t190 * t1289;
    let t1344 = 4.0 * t681 * t1342;
    let t1347 = piecewise3(t151, 0.0, 4.0 / 3.0 * t78 * t1289);
    let t1350 = piecewise3(t155, 0.0, -4.0 / 3.0 * t81 * t1289);
    let t1351 = t1347 + t1350;
    let t1352 = t150 * t1351;
    let t1353 = t1352 * t190;
    let t1354 = t1351 * t162;
    let t1356 = 0.19751673498613801407e-1 * t1354 * t187;
    let t1359 = piecewise3(t151, 0.0, 2.0 / 3.0 * t741 * t1289);
    let t1362 = piecewise3(t155, 0.0, -2.0 / 3.0 * t745 * t1289);
    let t1364 = t1359 / 2.0 + t1362 / 2.0;
    (t1342, t1344, t1351, t1352, t1353, t1354, t1356, t1364)
}
