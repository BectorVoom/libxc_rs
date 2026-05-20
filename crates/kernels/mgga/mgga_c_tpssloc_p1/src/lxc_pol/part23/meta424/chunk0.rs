//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1252/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252<F: Float>(t10480: F, t21391: F, t248: F, t3101: F, t1041: F, t10457: F, t21118: F, t1020: F, t21595: F, t14511: F, t17655: F, t10883: F, t21403: F) -> (F, F, F, F, F) {
    let t70227 = t10480 * t248 * t3101 * t21391;
    let t70239 = t1041 * t248 * t10457 * t21118;
    let t70346 = t1020 * t248 * t3101 * t21595;
    let t70351 = t14511 * t17655;
    let t70363 = t10883 * t248 * t3101 * t21403;
    (t70227, t70239, t70346, t70351, t70363)
}
