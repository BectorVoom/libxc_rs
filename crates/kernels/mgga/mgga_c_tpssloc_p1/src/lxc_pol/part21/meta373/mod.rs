//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta373<F: Float>(t13716: F, t942: F, t951: F, t959: F, t2940: F, t4489: F, t10523: F, t1580: F, t2933: F, t1543: F, t2791: F, t2794: F) -> (F, F, F, F, F, F, F) {
        let (t13718, t13720, t13722, t13724, t13726, t13727, t13729) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1822::<F>(t13716, t942, t951, t959, t2940, t4489, t10523, t1580, t2933, t1543, t2791, t2794);
    (t13718, t13720, t13722, t13724, t13726, t13727, t13729)
}
