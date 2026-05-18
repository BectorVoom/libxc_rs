//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1135/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1135<F: Float>(t6552: F, t6637: F, t776: F, t81658: F, t1888: F, t232: F, t40955: F, t6646: F, t23110: F, t23176: F, t23185: F, t252: F, t9660: F) -> (F, F, F, F) {
    let t81661 = t6552 * t6637 * t81658 * t776;
    let t81667 = t1888 * t6646 * t40955 * t232;
    let t81670 = t23185 * t23110 * t23176;
    let t81672 = t252 * t9660;
    (t81661, t81667, t81670, t81672)
}
