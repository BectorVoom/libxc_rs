//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1308/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1308<F: Float>(t23155: F, t23168: F, t6552: F, t6637: F, t6638: F, t9516: F, t22893: F, t23158: F, t23164: F, t22715: F, t6551: F, t6640: F) -> (F, F, F, F, F) {
    let t81623 = t23168 * t23155;
    let t81627 = t6552 * t6637 * t6638 * t9516;
    let t81630 = t23164 * t22893 * t23158;
    let t81632 = t22715 * t6551;
    let t81633 = t81632 * t6640;
    (t81623, t81627, t81630, t81632, t81633)
}
