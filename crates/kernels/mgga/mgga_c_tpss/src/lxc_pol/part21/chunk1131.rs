//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1131/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1131<F: Float>(t1321: F, t2061: F, t2105: F, t93: F, t1334: F, t2023: F, t3509: F, t600: F, t3533: F, t1333: F, t2074: F, t7594: F, t2073: F, t3532: F, t640: F, t2100: F, t3508: F) -> (F, F, F, F, F, F, F, F) {
    let t13136 = t1321 * t2061;
    let t13146 = t93 * t2105;
    let t13154 = t2023 * t1334;
    let t13157 = 4.0 / 3.0 * t600 * t3509;
    let t13159 = 2.0 / 3.0 * t600 * t3533;
    let t13161 = t7594 * t1333 * t2074;
    let t13164 = t2073 * t3532;
    let t13165 = t13164 * t640;
    let t13168 = t3508 * t2100;
    (t13136, t13146, t13154, t13157, t13159, t13161, t13165, t13168)
}
