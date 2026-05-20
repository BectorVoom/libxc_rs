//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2241/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2241<F: Float>(t28406: F, t814: F, t234: F, t5631: F, t6552: F, t6637: F, t776: F, t16758: F, t22986: F, t2647: F, t6646: F, t5593: F, t81865: F) -> (F, F, F, F) {
    let t98592 = t814 * t28406;
    let t98598 = t234 * t5631;
    let t98601 = t6552 * t6637 * t98598 * t776;
    let t98608 = t22986 * t6646 * t16758 * t2647;
    let t98610 = t81865 * t5593;
    (t98592, t98601, t98608, t98610)
}
