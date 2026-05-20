//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2279/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2279<F: Float>(t39563: F, t39585: F, t39590: F, t39593: F, t46331: F, t46334: F, t46336: F, t46338: F, t46339: F, t46345: F, t46349: F, t46353: F, t46355: F, t46361: F, t46367: F, t46370: F, t46372: F) -> F {
    let t47146 = t39563 - t46331 + t46334 - t39585 + t39590 + t46336 + t46338 + t46339 - t39593 + t46345 + t46349 + t46353 + t46355 + t46361 + t46367 + t46370 - t46372;
    t47146
}
