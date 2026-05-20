//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta465<F: Float>(t23024: F, t23189: F, t858: F, t225: F, t2717: F, t2719: F, t6553: F, t1880: F, t1902: F, t2591: F, t252: F, t794: F) -> (F, F, F, F, F, F, F) {
        let (t23190, t23191, t23196, t23197, t23198, t23202, t23204) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1793::<F>(t23024, t23189, t858, t225, t2717, t2719, t6553, t1880, t1902, t2591, t252, t794);
    (t23190, t23191, t23196, t23197, t23198, t23202, t23204)
}
