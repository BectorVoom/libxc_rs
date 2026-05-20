//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1755/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1755<F: Float>(t133: F, t1995: F, t6933: F, t80953: F, t22803: F, t6604: F, t2229: F, t583: F, t60: F, t22816: F, t22818: F, t22764: F, t3777: F) -> (F, F, F, F, F) {
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80958 = t22803 * t6604;
    let t80967 = F::new(1.0) / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    let t80991 = t3777 * t22764;
    (t80956, t80958, t80967, t80970, t80991)
}
