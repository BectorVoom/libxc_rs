//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2124/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2124<F: Float>(t25083: F, t2617: F, t4184: F, t13244: F, t25084: F, t25064: F, t81788: F, t13193: F, t6621: F, t13198: F, t23097: F, t232: F, t46565: F, t815: F) -> (F, F, F, F, F, F) {
    let t87379 = t2617 * t25083 * t4184;
    let t87381 = t25084 * t13244;
    let t87387 = t81788 * t25064;
    let t87389 = t6621 * t13193;
    let t87391 = t6621 * t13198;
    let t87395 = t23097 * t815 * t46565 * t232;
    (t87379, t87381, t87387, t87389, t87391, t87395)
}
