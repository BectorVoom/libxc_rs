//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2499/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2499<F: Float>(t2570: F, t2585: F, t4255: F, t46853: F, t13326: F, t9638: F, t2628: F, t2691: F, t4184: F, t812: F, t1512: F, t41362: F) -> (F, F, F, F) {
    let t46855 = t2585 * t2570 * t46853 * t4255;
    let t46870 = t9638 * t13326;
    let t46874 = t812 * t2628 * t2691 * t4184;
    let t46876 = t41362 * t1512;
    (t46855, t46870, t46874, t46876)
}
