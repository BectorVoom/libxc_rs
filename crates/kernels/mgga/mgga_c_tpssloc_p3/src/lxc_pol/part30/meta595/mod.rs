//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1976;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta595<F: Float>(t12328: F, t2003: F, t12248: F, t59: F, t1336: F, t2690: F, t6943: F, t1354: F, t22865: F, t6604: F, t6937: F, t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F, t2229: F, t583: F, t60: F, t22816: F, t22818: F, t22764: F, t3777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80900, t80901, t80914, t80915, t80939, t80940, t80953) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1976::<F>(t12328, t2003, t12248, t59, t1336, t2690, t6943, t1354, t22865, t6604, t6937, t22811, t61);
        let (t80957, t80958, t80967, t80971, t80991) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1977::<F>(t133, t1995, t6933, t80953, t22803, t6604, t2229, t583, t60, t22816, t22818, t22764, t3777);
    (t80900, t80901, t80914, t80915, t80939, t80940, t80953, t80957, t80958, t80967, t80971, t80991)
}
