//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta570<F: Float>(t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t11064: F, t42332: F, t11058: F, t1014: F, t42340: F, t42341: F, t3127: F) -> (F, F, F, F, F, F) {
        let (t43385, t43398, t43470, t43473, t43503, t43515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2078::<F>(t10475, t42342, t42345, t2770, t283, t11064, t42332, t11058, t1014, t42340, t42341, t3127);
    (t43385, t43398, t43470, t43473, t43503, t43515)
}
