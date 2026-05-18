//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1098/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1098<F: Float>(t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F, t6604: F, t2229: F, t583: F, t60: F, t22816: F, t22818: F) -> (F, F, F, F, F) {
    let t80953 = F::new(1.0) / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80958 = t22803 * t6604;
    let t80967 = F::new(1.0) / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    (t80953, t80956, t80958, t80967, t80970)
}
