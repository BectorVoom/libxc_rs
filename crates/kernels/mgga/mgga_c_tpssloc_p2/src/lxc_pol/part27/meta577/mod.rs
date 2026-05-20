//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2025;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta577<F: Float>(t22773: F, t22779: F, t22865: F, t6604: F, t6937: F, t22776: F, t22811: F, t61: F, t133: F, t1995: F, t6933: F, t22803: F, t22829: F, t2229: F, t583: F, t60: F, t22816: F, t22818: F, t22765: F, t3858: F, t22764: F, t3777: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80922, t80939, t80940, t80943, t80953, t80957, t80958) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2025::<F>(t22773, t22779, t22865, t6604, t6937, t22776, t22811, t61, t133, t1995, t6933, t22803);
        let (t80959, t80967, t80971, t80989, t80991) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2026::<F>(t22829, t80958, t2229, t583, t60, t1995, t22816, t22818, t22765, t3858, t22764, t3777);
    (t80922, t80939, t80940, t80943, t80953, t80957, t80958, t80959, t80967, t80971, t80989, t80991)
}
