//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1139/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139<F: Float>(t2535: F, t9716: F, t39382: F, t761: F, t2531: F, t9713: F, t39302: F, t39563: F, t39585: F, t39590: F, t39593: F, t40818: F, t41252: F, t41254: F, t172: F, t763: F, t9915: F) -> (F, F, F, F, F, F) {
    let t41255 = t9716 * t2535;
    let t41256 = 0.35089341735807877242e1 * t41255;
    let t41258 = 0.91082604192152556044e5 * t761 * t39382;
    let t41259 = t2531 * t9713;
    let t41260 = 0.23392894490538584828e1 * t41259;
    let t41262 = 0.5848223622634646207e0 * t761 * t39302;
    let t41263 = t39563 - t40818 - t39585 + t39590 + t41252 - t39593 + t41254 - t41256 - t41258 - t41260 - t41262;
    let t41265 = t9915 * t172 * t763;
    (t41256, t41258, t41260, t41262, t41263, t41265)
}
