//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2234/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2234<F: Float>(t4199: F, t9722: F, t12887: F, t172: F, t763: F, t12858: F, t2535: F, t40794: F, t40804: F, t40806: F, t12606: F, t707: F, t751: F) -> (F, F, F, F, F, F, F) {
    let t46302 = t4199 * t9722;
    let t46303 = F::cast_from(0.10389515463408878255e3_f64) * t46302;
    let t46308 = t12887 * t172 * t763;
    let t46309 = F::cast_from(0.17544670867903938621e1_f64) * t46308;
    let t46310 = t12858 * t2535;
    let t46311 = F::cast_from(0.17544670867903938621e1_f64) * t46310;
    let t46313 = F::cast_from(0.48796115851357829289e-1_f64) * t40794;
    let t46314 = F::cast_from(0.97592231702715658578e-1_f64) * t40804;
    let t46315 = F::cast_from(0.14447919941302971323e1_f64) * t40806;
    let t46317 = t707 * t751 * t12606;
    (t46303, t46309, t46311, t46313, t46314, t46315, t46317)
}
