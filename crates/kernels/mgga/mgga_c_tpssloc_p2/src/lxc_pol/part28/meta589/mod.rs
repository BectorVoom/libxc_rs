//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta589<F: Float>(t25060: F, t6547: F, t1880: F, t23237: F, t25216: F, t25192: F, t81651: F, t82074: F, t6552: F, t6555: F, t87782: F, t23270: F, t25038: F, t25191: F, t87036: F, t25054: F, t23196: F, t25224: F, t23030: F, t25205: F, t23164: F, t7479: F, t82133: F, t82124: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87804, t87822, t87835, t87861, t87866) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1883::<F>(t25060, t6547, t1880, t23237, t25216, t25192, t81651, t82074, t6552, t6555, t87782, t23270, t25038, t25191, t87036);
        let (t87873, t87893, t87898, t87901, t87904) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1884::<F>(t25054, t81651, t82074, t1880, t23196, t25224, t23030, t25205, t23164, t7479, t82133, t6552, t82124);
    (t87804, t87822, t87835, t87861, t87866, t87873, t87893, t87898, t87901, t87904)
}
