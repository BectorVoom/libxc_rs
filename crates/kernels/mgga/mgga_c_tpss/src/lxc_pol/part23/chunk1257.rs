//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1257/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1257<F: Float>(t31464: F, t5965: F, t19191: F, t7690: F, t1859: F, t1981: F, t1985: F, t31450: F, t7682: F, t19218: F, t38: F, t55: F, t7750: F, t1860: F, t62020: F, t61942: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63495 = t31464 * t5965;
    let t63498 = t7690 * t19191;
    let t63506 = t1981 * t1985 * t1859;
    let t63521 = t31450 * t5965;
    let t63530 = t7682 * t19191;
    let t63534 = t1981 * t38 * t19218;
    let t63556 = t55 * t7750;
    let t63587 = t1860 * t62020;
    let t63590 = t1860 * t61942;
    (t63495, t63498, t63506, t63521, t63530, t63534, t63556, t63587, t63590)
}
