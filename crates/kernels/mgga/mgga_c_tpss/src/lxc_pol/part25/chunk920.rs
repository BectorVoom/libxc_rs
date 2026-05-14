//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 920/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk920<F: Float>(t13485: F, t640: F, t3508: F, t3532: F, t4670: F, t600: F, t2073: F, t4669: F, t4649: F, t7613: F, t633: F, t1324: F, t2: F, t555: F, t2083: F, t4577: F) -> (F, F, F, F, F, F, F) {
    let t13486 = t13485 * t640;
    let t13489 = t3508 * t3532;
    let t13492 = t600 * t4670;
    let t13494 = t2073 * t4669;
    let t13495 = t13494 * t640;
    let t13500 = t7613 * t4649;
    let t13501 = t13500 * t633;
    let t13504 = t1324 * t2;
    let t13505 = t13504 * t555;
    let t13510 = t2083 * t4577;
    (t13486, t13489, t13492, t13495, t13501, t13505, t13510)
}
