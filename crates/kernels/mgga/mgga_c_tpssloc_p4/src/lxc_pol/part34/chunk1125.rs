//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1125/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1125<F: Float>(t22716: F, t7741: F, t22724: F, t26436: F, t7736: F, t80854: F, t81064: F, t22642: F, t22690: F, t26395: F, t22863: F, t7737: F) -> (F, F, F, F, F) {
    let t90868 = t22716 * t7741;
    let t90900 = t22724 * t26436;
    let t90980 = t81064 * t80854 * t7736;
    let t90993 = t22642 * t22690 * t26395;
    let t91000 = t22863 * t7737;
    (t90868, t90900, t90980, t90993, t91000)
}
