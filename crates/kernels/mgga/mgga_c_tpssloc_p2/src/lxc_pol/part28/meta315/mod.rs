//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta315<F: Float>(t1128: F, t3324: F, t1124: F, t3356: F, t3355: F, t432: F, t427: F, t1094: F, t3263: F, t3395: F, t3403: F, t11135: F) -> (F, F, F, F, F, F) {
        let (t11410, t11415, t11420, t11424, t11433, t11444) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1240::<F>(t1128, t3324, t1124, t3356, t3355, t432, t427, t1094, t3263, t3395, t3403, t11135);
    (t11410, t11415, t11420, t11424, t11433, t11444)
}
