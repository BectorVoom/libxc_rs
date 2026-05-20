//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta517<F: Float>(t1395: F, t671: F, t1372: F, t794: F, t6897: F, t6907: F, t213: F, t225: F, t22674: F, t22892: F, t22916: F, t22716: F, t6908: F) -> (F, F, F, F, F, F) {
        let (t66940, t80645, t80647, t80650, t80659, t80663) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1765::<F>(t1395, t671, t1372, t794, t6897, t6907, t213, t225, t22674, t22892, t22916, t22716, t6908);
    (t66940, t80645, t80647, t80650, t80659, t80663)
}
