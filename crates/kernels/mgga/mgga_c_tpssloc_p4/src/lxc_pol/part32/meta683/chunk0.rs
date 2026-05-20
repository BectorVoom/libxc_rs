//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2123/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2123<F: Float>(t19440: F, t71: F, t33: F, t55880: F, t5441: F, t645: F, t72: F, t5389: F, t641: F, t12568: F, t1410: F, t27960: F) -> (F, F, F, F, F, F) {
    let t96379 = t71 * t19440;
    let t96383 = t55880 * t33;
    let t96393 = t72 * t5441 * t645;
    let t96403 = t72 * t641 * t5389;
    let t96406 = t12568 * t1410;
    let t96418 = t72 * t27960 * t645;
    (t96379, t96383, t96393, t96403, t96406, t96418)
}
