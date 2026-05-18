//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1243/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1243<F: Float>(t5410: F, t5721: F, t5415: F, t5420: F, t5728: F, t5424: F, t1705: F, t5427: F, t935: F, t1639: F, t1656: F, t520: F) -> (F, F, F, F, F, F, F) {
    let t21044 = t5721 * t5410;
    let t21046 = t5721 * t5415;
    let t21048 = t5728 * t5420;
    let t21050 = t5728 * t5424;
    let t21060 = t1705 * t5427;
    let t21061 = t21060 * t935;
    let t21074 = t1656 * t1639 * t520;
    (t21044, t21046, t21048, t21050, t21060, t21061, t21074)
}
