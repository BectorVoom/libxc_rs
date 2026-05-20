//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta614<F: Float>(t16261: F, t26309: F, t22832: F, t5234: F, t3809: F, t16405: F, t22833: F, t16387: F, t16275: F, t16271: F, t1336: F, t22759: F, t5252: F, t836: F) -> (F, F, F, F, F, F, F) {
        let (t91098, t91101, t91103, t91105, t91107, t91109, t91113) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1929::<F>(t16261, t26309, t22832, t5234, t3809, t16405, t22833, t16387, t16275, t16271, t1336, t22759, t5252, t836);
    (t91098, t91101, t91103, t91105, t91107, t91109, t91113)
}
