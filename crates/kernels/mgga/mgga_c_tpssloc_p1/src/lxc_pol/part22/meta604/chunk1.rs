//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2127/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2127<F: Float>(t49690: F, t14202: F, t3117: F, t10890: F, t14507: F, t3038: F, t49650: F, t1020: F, t10508: F, t248: F, t4650: F, t13965: F, t3109: F) -> (F, F, F, F, F, F) {
    let t49691 = t49690 / F::new(4608.0);
    let t49692 = t3117 * t14202;
    let t49693 = t49692 / F::new(6912.0);
    let t49743 = t14507 * t10890;
    let t49771 = t49650 * t3038;
    let t49818 = t1020 * t248 * t10508 * t4650;
    let t49819 = t49818 / F::new(4608.0);
    let t49831 = t3109 * t13965;
    (t49691, t49693, t49743, t49771, t49819, t49831)
}
