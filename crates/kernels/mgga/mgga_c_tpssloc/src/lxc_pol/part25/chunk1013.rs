//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1013/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1013<F: Float>(t22893: F, t23158: F, t23164: F, t22715: F, t6551: F, t6640: F, t23153: F, t2553: F, t6552: F, t6637: F, t117: F, t4179: F, t6559: F, t23036: F, t10094: F, t1888: F, t22996: F) -> (F, F, F, F, F, F, F) {
    let t81630 = t23164 * t22893 * t23158;
    let t81632 = t22715 * t6551;
    let t81633 = t81632 * t6640;
    let t81637 = t6552 * t6637 * t23153 * t2553;
    let t81640 = t6559 * t4179 * t117;
    let t81642 = t81640 * t22893 * t23036;
    let t81645 = t1888 * t22996 * t10094;
    (t81630, t81632, t81633, t81637, t81640, t81642, t81645)
}
