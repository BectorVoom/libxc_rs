//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 195/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk195<F: Float>(t53: F, t60: F, t521: F, t912: F, t50: F, t57: F, t280: F, t814: F, t525: F, t921: F, t62: F, t284: F, t68: F, t183: F, t155: F, t421: F, t577: F, t381: F, t578: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1395 = t912 * t521;
    let t1398 = t57 * t50;
    let t1402 = piecewise3(t54, 0.0, 4.0 / 9.0 * t1395 * t280 + 8.0 / 3.0 * t1398 * t814);
    let t1403 = t921 * t525;
    let t1406 = t62 * t50;
    let t1410 = piecewise3(t61, 0.0, 4.0 / 9.0 * t1403 * t284 - 8.0 / 3.0 * t1406 * t814);
    let t1411 = t1402 + t1410;
    let t1412 = t1411 * t68;
    let t1413 = t1412 * t183;
    let t1414 = t155 * t1413;
    let t1415 = t577 * t421;
    let t1416 = t155 * t1415;
    let t1417 = t381 * t578;
    (t1411, t1412, t1414, t1416, t1417)
}
