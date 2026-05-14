//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 715/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk715<F: Float>(t1334: F, t600: F, t1333: F, t2073: F, t640: F, t1324: F, t2083: F, t633: F, t100: F, t2: F, t555: F, t1329: F, t2091: F, t636: F, t108: F, t105: F, t1325: F, t1327: F, t631: F, t637: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3506 = t600 * t1334;
    let t3508 = t2073 * t1333;
    let t3509 = t3508 * t640;
    let t3514 = t2083 * t1324;
    let t3515 = t3514 * t633;
    let t3518 = t100 * t2;
    let t3519 = t3518 * t555;
    let t3524 = t2091 * t1329;
    let t3525 = t3524 * t636;
    let t3528 = t108 * t2;
    let t3529 = t3528 * t555;
    let t3532 = -25.0 / 9.0 * t631 * t1325 + 10.0 / 9.0 * t97 * t3515 + 5.0 / 3.0 * t97 * t3519 - 25.0 / 9.0 * t1327 * t637 + 10.0 / 9.0 * t105 * t3525 - 5.0 / 3.0 * t105 * t3529;
    (t3506, t3508, t3509, t3514, t3515, t3518, t3519, t3524, t3528, t3532)
}
