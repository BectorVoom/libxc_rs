//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 423/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk423<F: Float>(t1457: F, t912: F, t1289: F, t929: F, t926: F, t1413: F, t1427: F, t1453: F, t1455: F) -> (F, F, F, F) {
    let t1459 = F::cast_from(0.5848223622634646207e0_f64) * t912 * t1457;
    let t1460 = t929 * t1289;
    let t1461 = t926 * t1460;
    let t1464 = -t1413 + t1427 + t1453 + t1455 - t1459;
    (t1459, t1460, t1461, t1464)
}
