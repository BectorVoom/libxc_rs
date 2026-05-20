//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta622<F: Float>(t11801: F, t7345: F, t11708: F, t24728: F, t11713: F, t11715: F, t11717: F, t2131: F, t82985: F, t24727: F, t24732: F, t7337: F, sigma2: F) -> (F, F, F, F, F, F, F) {
        let (t86136, t86140, t86146, t86154, t86164, t86167, t86171) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064::<F>(t11801, t7345, t11708, t24728, t11713, t11715, t11717, t2131, t82985, t24727, t24732, t7337, sigma2);
    (t86136, t86140, t86146, t86154, t86164, t86167, t86171)
}
