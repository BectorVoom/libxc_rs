//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk705;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta132<F: Float>(t1203: F, t1222: F, t221: F, t3426: F, t456: F, t1197: F, t135: F, t1174: F, t1176: F, t3247: F, t3242: F, t3439: F, t121: F, t486: F) -> (F, F, F, F, F, F, F, F) {
        let (t3543, t3545, t3547, t3548, t3549, t3555, t3560) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk705::<F>(t1203, t1222, t221, t3426, t456, t1197, t135, t1174, t1176, t3247, t3242, t3439);
        let t3570 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk706::<F>(t121, t486);
    (t3543, t3545, t3547, t3548, t3549, t3555, t3560, t3570)
}
