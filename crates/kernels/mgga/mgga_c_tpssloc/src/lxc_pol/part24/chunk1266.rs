//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1266/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1266<F: Float>(t80956: F, t22803: F, t6604: F, t22829: F, t1339: F, t26288: F, t54542: F, t550: F, t2229: F, t583: F, t60: F, t1995: F, t22816: F, t22818: F) -> (F, F, F, F, F) {
    let t80957 = F::new(0.69792532988666768264e-2) * t80956;
    let t80958 = t22803 * t6604;
    let t80959 = t80958 * t22829;
    let t80963 = t26288 * t1339 * t54542 * t550;
    let t80967 = F::new(1.0) / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    (t80957, t80959, t80963, t80967, t80970)
}
