//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2235/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2235<F: Float>(t46317: F, t40808: F, t2749: F, t776: F, t12915: F, t2522: F, t39549: F, t40797: F, t40799: F, t40801: F, t40803: F, t46313: F, t46314: F, t46315: F) -> (F, F, F) {
    let t46318 = F::new(12.0) * t46317;
    let t46319 = F::new(12.0) * t40808;
    let t46320 = t776 * t2749;
    let t46324 = F::new(18.0) * t12915 * t2522 * t46320 + t39549 + t40797 + t40799 + t40801 - t40803 + t46313 - t46314 + t46315 + t46318 + t46319;
    (t46318, t46319, t46324)
}
