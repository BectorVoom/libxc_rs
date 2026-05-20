//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1210;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta319<F: Float>(t12110: F, t2375: F, t3684: F, t9882: F, t9888: F, t9885: F, t3824: F, t588: F, t1287: F, t2225: F, t1284: F, t2516: F, t17: F, t521: F, t9861: F, t3826: F, t592: F, t1285: F, t2371: F, t3691: F, t1294: F, t9494: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12111, t12114, t12116, t12118, t12120, t12123, t12129) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1210::<F>(t12110, t2375, t3684, t9882, t9888, t9885, t3824, t588, t1287, t2225, t1284, t2516);
        let (t12130, t12133, t12134, t12136, t12138, t12141) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1211::<F>(t12129, t17, t521, t9861, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494);
    (t12111, t12114, t12116, t12118, t12120, t12123, t12130, t12133, t12134, t12136, t12138, t12141)
}
