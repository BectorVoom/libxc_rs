//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta368<F: Float>(t13213: F, t13268: F, t13331: F, t13375: F, t218: F, t1509: F, t852: F, t829: F, t252: F, t4233: F, t4182: F, t2684: F, t4282: F) -> (F, F, F, F, F, F, F) {
        let (t13377, t13378, t13380, t13381, t13384, t13385, t13388) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1476::<F>(t13213, t13268, t13331, t13375, t218, t1509, t852, t829, t252, t4233, t4182, t2684, t4282);
    (t13377, t13378, t13380, t13381, t13384, t13385, t13388)
}
