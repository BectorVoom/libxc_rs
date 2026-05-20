//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1774;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta548<F: Float>(t1878: F, t845: F, t2230: F, t23076: F, t213: F, t200: F, t23075: F, t598: F, t23034: F, t6546: F, t131: F, t23143: F, t6649: F) -> (F, F, F, F, F, F, F) {
        let (t81959, t81962, t81963, t81968, t81979, t81982, t82011) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1774::<F>(t1878, t845, t2230, t23076, t213, t200, t23075, t598, t23034, t6546, t131, t23143, t6649);
    (t81959, t81962, t81963, t81968, t81979, t81982, t82011)
}
