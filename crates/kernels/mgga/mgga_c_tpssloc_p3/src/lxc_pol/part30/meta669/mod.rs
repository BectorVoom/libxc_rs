//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2097;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta669<F: Float>(t5303: F, t80820: F, t22783: F, t5310: F, t1827: F, t80914: F, t1811: F, t80775: F, t7709: F, t80766: F, t22797: F, t5227: F, t22804: F, t26277: F, t225: F, t26221: F, t22674: F, t22892: F, t26189: F, t26329: F, t26229: F, t22724: F, t26344: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91365, t91387, t91394, t91398, t91400, t91402) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2097::<F>(t5303, t80820, t22783, t5310, t1827, t80914, t1811, t80775, t7709, t80766, t22797, t5227);
        let (t91403, t91404, t91441, t91487, t91488, t91491, t91531) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098::<F>(t91402, t22804, t26277, t225, t26221, t22674, t22892, t26189, t26329, t26229, t22724, t26344);
    (t91365, t91387, t91394, t91398, t91400, t91403, t91404, t91441, t91487, t91488, t91491, t91531)
}
