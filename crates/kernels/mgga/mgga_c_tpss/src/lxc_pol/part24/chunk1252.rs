//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1252/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1252<F: Float>(t1395: F, t18007: F, t18005: F, t6134: F, t19733: F, t5570: F, t6130: F, t768: F, t1702: F, t8275: F, t30: F, t31814: F, t19797: F, t2436: F, t198: F, t206: F, t6148: F) -> (F, F, F, F, F, F, F, F) {
    let t64028 = t18007 * t1395;
    let t64060 = t6134 * t18005;
    let t64135 = t19733 * t5570;
    let t64159 = t768 * t6130;
    let t64163 = t8275 * t1702;
    let t64247 = t31814 * t30;
    let t64277 = t19797 * t2436;
    let t64284 = t198 * t206 * t6148;
    (t64028, t64060, t64135, t64159, t64163, t64247, t64277, t64284)
}
