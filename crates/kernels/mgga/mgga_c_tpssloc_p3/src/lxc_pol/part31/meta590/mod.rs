//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1834;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta590<F: Float>(t1361: F, t22690: F, t22792: F, t5187: F, t1307: F, t7708: F, t80840: F, t90787: F, t26245: F, t80783: F, t22897: F, t6925: F, t26302: F, t80958: F, t22779: F, t26323: F, t1336: F, t242: F, t80901: F, t5303: F, t80820: F, t80915: F, t22783: F, t5310: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91327, t91344, t91346, t91351) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1834::<F>(t1361, t22690, t22792, t5187, t1307, t7708, t80840, t90787, t26245, t80783, t22897, t6925);
        let (t91356, t91358, t91361, t91364, t91383, t91386) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1835::<F>(t26302, t80958, t22779, t26323, t1336, t242, t80901, t5303, t80820, t80915, t22783, t5310);
    (t91327, t91344, t91346, t91351, t91356, t91358, t91361, t91364, t91383, t91386)
}
