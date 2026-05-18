//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1372/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1372<F: Float>(t11791: F, t7345: F, t11820: F, t7339: F, t11698: F, t24741: F, t2132: F, t24746: F, t86202: F, t11754: F, t7310: F, t86197: F) -> (F, F, F, F, F, F) {
    let t86348 = t7345 * t11791;
    let t86350 = t7339 * t11820;
    let t86354 = t24741 * t11698;
    let t86357 = t2132 * t86202 * t24746;
    let t86365 = t7310 * t11754;
    let t86368 = t2132 * t86197 * t24746;
    (t86348, t86350, t86354, t86357, t86365, t86368)
}
