//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1263/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1263<F: Float>(t63990: F, t136: F, t1693: F, t799: F, t19725: F, t219: F, t1395: F, t18007: F, t18005: F, t6134: F, t19733: F, t5570: F, t6130: F, t768: F, t1702: F, t8275: F) -> (F, F, F, F, F, F, F, F) {
    let t63991 = 7.0 / 24.0 * t63990;
    let t63993 = t1693 * t799 * t136;
    let t64016 = t19725 * t219;
    let t64028 = t18007 * t1395;
    let t64060 = t6134 * t18005;
    let t64135 = t19733 * t5570;
    let t64159 = t768 * t6130;
    let t64163 = t8275 * t1702;
    (t63991, t63993, t64016, t64028, t64060, t64135, t64159, t64163)
}
