//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta575<F: Float>(t1254: F, t5091: F, t1176: F, t1714: F, t1395: F, t671: F, t1372: F, t794: F, t6897: F, t6907: F, t213: F, t225: F) -> (F, F, F, F, F, F) {
        let (t64447, t64825, t66940, t80645, t80647, t80650) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1992::<F>(t1254, t5091, t1176, t1714, t1395, t671, t1372, t794, t6897, t6907, t213, t225);
    (t64447, t64825, t66940, t80645, t80647, t80650)
}
