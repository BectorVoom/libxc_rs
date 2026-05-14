//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 800/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk800<F: Float>(t21033: F, t858: F, t20936: F, t252: F, t1492: F, t5631: F, t1527: F, t5636: F, t10110: F, t5657: F, t2718: F, t1519: F, t5558: F, t21013: F, t218: F, t1528: F, t17052: F, t17090: F, t17092: F, t259: F, t4147: F, t4268: F, t5637: F, t5658: F, t855: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21034 = t858 * t21033;
    let t21036 = t20936 * t252;
    let t21038 = t1492 * t5631;
    let t21049 = t5636 * t1527;
    let t21050 = t10110 * t21049;
    let t21053 = t1527 * t5657;
    let t21054 = t2718 * t21053;
    let t21061 = t5558 * t1519;
    let t21064 = t218 * t21013;
    let t21066 = -3.0 * t1528 * t17052 - 3.0 * t1528 * t17090 - 6.0 * t1528 * t17092 - t21034 * t855 + t21036 * t259 + 3.0 * t21038 * t259 - 6.0 * t21050 * t855 + 6.0 * t21054 * t855 + 3.0 * t21061 * t259 + t21064 * t259 + 6.0 * t4147 * t5637 - 3.0 * t4147 * t5658 + 6.0 * t4268 * t5637 - 3.0 * t4268 * t5658;
    (t21034, t21036, t21038, t21049, t21050, t21053, t21054, t21061, t21064, t21066)
}
