//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1225/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1225<F: Float>(t11791: F, t7345: F, t11820: F, t7339: F, t11698: F, t24741: F, t2132: F, t24746: F, t86202: F, t11754: F, t7310: F, t86197: F, t11761: F, t11850: F, t11858: F, t2140: F, t24699: F, t24749: F, t488: F, t7316: F, t7321: F) -> (F,) {
    let t86348 = t7345 * t11791;
    let t86350 = t7339 * t11820;
    let t86354 = t24741 * t11698;
    let t86357 = t2132 * t86202 * t24746;
    let t86365 = t7310 * t11754;
    let t86368 = t2132 * t86197 * t24746;
    let t86373 = t86348 / 3456.0 - t86350 / 2304.0 + t7310 * t11761 / 36.0 - t86354 / 576.0 - 0.30279567070605293142e-3 * t86357 - t7310 * t11850 / 48.0 + 0.30279567070605293142e-3 * t24749 * t7321 + 0.30279567070605293142e-3 * t7316 * t24699 + t86365 / 216.0 - 0.30279567070605293142e-3 * t86368 + t11858 * t2140 * t488 / 1536.0;
    (t86373,)
}
