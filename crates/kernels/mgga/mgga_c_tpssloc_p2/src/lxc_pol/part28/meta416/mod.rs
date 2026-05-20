//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1588;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta416<F: Float>(t10: F, t2229: F, t60: F, t1995: F, t116: F, t117: F, t67: F, t1999: F, t794: F, t61: F, t9222: F, t133: F, t6933: F, t6604: F, t6925: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22811, t22813, t22814, t22816) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1588::<F>(t10, t2229, t60, t1995, t116, t117, t67);
        let (t22818, t22819, t22822, t22823, t22825, t22827) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1589::<F>(t22814, t22816, t1999, t794, t61, t9222, t1995, t133, t6933, t6604, t6925);
    (t22811, t22813, t22814, t22816, t22818, t22819, t22822, t22823, t22825, t22827)
}
