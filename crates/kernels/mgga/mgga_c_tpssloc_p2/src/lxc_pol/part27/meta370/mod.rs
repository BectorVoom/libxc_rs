//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1525;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta370<F: Float>(t13716: F, t942: F, t951: F, t959: F, t2940: F, t4489: F, t10523: F, t1580: F, t2933: F, t1543: F, t2791: F, t2794: F, t4498: F, t2925: F, t4488: F, t1634: F, t3175: F, t10165: F, t1065: F, t4693: F, t3174: F, t2970: F, t4343: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13720, t13722, t13726, t13729) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1525::<F>(t13716, t942, t951, t959, t2940, t4489, t10523, t1580, t2933, t1543, t2791, t2794);
        let (t13731, t13734, t13735, t13736, t13742, t13743, t13748) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1526::<F>(t2940, t4498, t2925, t4488, t959, t1634, t3175, t10165, t1065, t4693, t3174, t2970, t4343);
    (t13720, t13722, t13726, t13729, t13731, t13734, t13735, t13736, t13742, t13743, t13748)
}
