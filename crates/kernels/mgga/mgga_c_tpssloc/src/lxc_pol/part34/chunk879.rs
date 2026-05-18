//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 879/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk879<F: Float>(t21033: F, t858: F, t20936: F, t252: F, t1492: F, t5631: F, t1527: F, t5636: F, t10110: F, t5657: F, t2718: F, t1519: F, t5558: F) -> (F, F, F, F, F, F, F, F) {
    let t21034 = t858 * t21033;
    let t21036 = t20936 * t252;
    let t21038 = t1492 * t5631;
    let t21049 = t5636 * t1527;
    let t21050 = t10110 * t21049;
    let t21053 = t1527 * t5657;
    let t21054 = t2718 * t21053;
    let t21061 = t5558 * t1519;
    (t21034, t21036, t21038, t21049, t21050, t21053, t21054, t21061)
}
