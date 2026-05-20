//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2099/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2099<F: Float>(t46291: F, t4199: F, t9722: F, t12858: F, t2535: F, t4205: F, t9868: F, t193: F, t776: F, t1409: F, t707: F, t9862: F) -> (F, F, F, F, F, F) {
    let t46292 = F::new(36.0) * t46291;
    let t46302 = t4199 * t9722;
    let t46310 = t12858 * t2535;
    let t46311 = F::cast_from(0.17544670867903938621e1_f64) * t46310;
    let t46335 = t4205 * t9868;
    let t46336 = F::new(12.0) * t46335;
    let t46341 = t193 * t776;
    let t46369 = t707 * t9862 * t1409;
    (t46292, t46302, t46311, t46336, t46341, t46369)
}
