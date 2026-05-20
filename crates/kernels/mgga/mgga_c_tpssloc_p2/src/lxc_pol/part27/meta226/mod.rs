//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1087;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta226<F: Float>(t25: F, t1799: F, t571: F, t3919: F, t1408: F, t3664: F, t2: F, t514: F, t584: F, t606: F, t1649: F, t3672: F, t517: F, zeta_threshold: F, t28: F, t1081: F, t157: F) -> (F, F, F, F, F, F, F) {
        let (t5127, t5131, t5134, t5137, t5141, t5142, t5145) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1087::<F>(t25, t1799, t571, t3919, t1408, t3664, t2, t514, t584, t606, t1649, t3672, t517, zeta_threshold);
        let t5151 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1088::<F>(t28, t1081, t5142, t5145, t584, t157, t5141, zeta_threshold);
    (t5127, t5131, t5134, t5137, t5142, t5145, t5151)
}
