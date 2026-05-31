//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 452/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk452<F: Float>(t40: F, t52: F, t1458: F, t510: F, t1409: F, t185: F, t707: F, t73: F, t76: F, t145: F, t157: F, t182: F, t767: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t1459 = t510 * t1458;
    let t1462 = t185 * t1409;
    let t1464 = F::cast_from(4.0_f64) * t707 * t1462;
    let t1467 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t1409);
    let t1470 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t1409);
    let t1471 = t1467 + t1470;
    let t1472 = t145 * t1471;
    let t1473 = t1472 * t185;
    let t1474 = t1471 * t157;
    let t1476 = F::cast_from(0.19751673498613801407e-1_f64) * t1474 * t182;
    let t1479 = piecewise3::<F>(t146, F::cast_from(0.0_f64), F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t1409);
    let t1482 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t1409);
    (t1459, t1462, t1464, t1471, t1472, t1473, t1474, t1476, t1479, t1482)
}
