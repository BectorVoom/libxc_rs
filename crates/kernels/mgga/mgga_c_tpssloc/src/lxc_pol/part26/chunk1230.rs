//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1230/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1230<F: Float>(t11553: F, t2121: F, t2123: F, t2122: F, t85628: F, t24574: F, t24884: F, t11154: F, t11613: F, t1186: F, t11928: F, t2155: F, t24582: F, t24595: F, t24893: F, t3471: F, t3487: F, t3631: F, t45345: F, t45355: F, t466: F, t498: F, t7283: F, t7286: F, t7295: F, t7356: F, t86376: F) -> (F,) {
    let t86451 = 0.30461741978670859935e-2 * t2121 * t11553 * t2123;
    let t86452 = t2122 * t85628;
    let t86456 = t24574 * t24884;
    let t86468 = -0.24674011002723396548e-1 * t7283 * t3471 * t7295 + 6.0 * t11928 * t7356 + 12.0 * t3487 * t24582 - 3.0 * t45345 * t2155 + 12.0 * t11613 * t7356 + t86451 + 0.24674011002723396548e-1 * t7283 * t1186 * t86452 - 0.27415567780803773942e-2 * t86456 + t466 * t86376 * t498 - 3.0 * t24893 * t3631 - 3.0 * t45355 * t2155 + 0.21932454224643019154e-1 * t7283 * t24595 * t7286 * t11154;
    (t86468,)
}
