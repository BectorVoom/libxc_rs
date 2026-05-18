//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 422/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk422<F: Float>(t1413: F, t1427: F, t1429: F, t1437: F, t1442: F, t1449: F, t294: F, t305: F, t877: F, t896: F, t1448: F, t895: F, t904: F) -> (F, F, F) {
    let t1453 = t294 * (-F::new(0.310907e-1) * t1429 * t305 + F::new(1.0) * t877 * t1437 + t1413 - t1427 - F::new(0.19751673498613801407e-1) * t1442 + F::new(0.5848223622634646207e0) * t896 * t1449);
    let t1455 = F::new(0.19751673498613801407e-1) * t294 * t1442;
    let t1457 = t895 * t1448 * t904;
    (t1453, t1455, t1457)
}
