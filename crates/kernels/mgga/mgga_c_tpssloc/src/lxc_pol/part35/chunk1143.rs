//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1143/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1143<F: Float>(t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F, t6604: F, t2229: F, t583: F, t60: F, t22816: F, t22818: F, t1336: F, t22759: F, t835: F, t12248: F) -> (F, F, F, F, F, F, F) {
    let t80953 = 1.0 / t61 / t22811;
    let t80956 = t80953 * t1995 * t133 * t6933;
    let t80957 = 0.69792532988666768264e-2 * t80956;
    let t80958 = t22803 * t6604;
    let t80967 = 1.0 / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    let t80971 = 0.43737152435318756759e-3 * t80970;
    let t80997 = t1336 * t22759 * t835;
    let t81027 = t6604 * t12248;
    (t80953, t80957, t80958, t80967, t80971, t80997, t81027)
}
