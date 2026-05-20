//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2485/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2485<F: Float>(t12858: F, t2535: F, t12606: F, t707: F, t751: F, t4205: F, t9868: F, t193: F, t776: F, t3966: F, t4194: F, t607: F, t750: F) -> (F, F, F, F, F) {
    let t46310 = t12858 * t2535;
    let t46317 = t707 * t751 * t12606;
    let t46335 = t4205 * t9868;
    let t46341 = t193 * t776;
    let t46348 = t4194 * t750 * t3966 * t607;
    (t46310, t46317, t46335, t46341, t46348)
}
