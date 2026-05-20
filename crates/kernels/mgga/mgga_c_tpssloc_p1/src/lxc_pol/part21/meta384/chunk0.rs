//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1845/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1845<F: Float>(t13985: F, t4593: F, t4582: F, t3132: F, t3069: F, t4669: F) -> (F, F, F, F, F) {
    let t13986 = t4593 * t13985;
    let t13987 = t4582 * t13986;
    let t13990 = t4593 * t3132;
    let t13991 = t4582 * t13990;
    let t13995 = t4669 * t3069;
    (t13986, t13987, t13990, t13991, t13995)
}
