//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1289/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1289<F: Float>(t104007: F, t104009: F, t104048: F, t104050: F, t22158: F, t22162: F, t22208: F, t22218: F, t22258: F, t24741: F, t27604: F, t27617: F, t6192: F, t6203: F, t6207: F, t6227: F, t6232: F, t7345: F, t95270: F, t95273: F, t95566: F, t95623: F, t95627: F) -> (F,) {
    let t109461 = -5.0 / 2592.0 * t7345 * t22208 - 5.0 / 432.0 * t27604 * t6203 - t27617 * t6207 / 768.0 - t7345 * t22258 / 384.0 + t104007 / 108.0 - t7345 * t22218 / 384.0 - t95623 * t6227 / 48.0 + t95627 * t6232 / 96.0 - t104009 / 768.0 + t95566 * t6192 / 72.0 + 5.0 / 2304.0 * t24741 * t22158 - t24741 * t22162 / 768.0 + t95270 * t6227 / 256.0 - t95273 * t6232 / 512.0 + t104048 / 768.0 - t104050 / 72.0;
    (t109461,)
}
