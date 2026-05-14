//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 465/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk465<F: Float>(t3440: F, t6119: F, t4904: F, t4919: F, t3455: F, t5392: F, t1177: F, t1178: F, t5398: F, t3464: F, t4770: F, t6012: F, t6015: F, t6018: F, t457: F, t460: F) -> (F, F, F, F, F, F) {
    let t6120 = t3440 * t6119;
    let t6123 = t4919 * t4904;
    let t6126 = t3455 * t5392;
    let t6127 = t1177 * t6126;
    let t6130 = t1178 * t5398;
    let t6131 = t1177 * t6130;
    let t6138 = -t3464 + 2.0 / 9.0 * t4770 + t6012 / 18.0 - t6015 / 3.0 - t6018 / 6.0;
    let t6139 = t457 * t6138;
    let t6140 = t6139 * t460;
    (t6120, t6123, t6127, t6131, t6138, t6140)
}
