//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2050;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta636<F: Float>(t87535: F, t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t25303: F, t6579: F, t23110: F, t4292: F, t25288: F, t234: F, t4265: F, t25237: F, t23168: F, t25307: F, t25287: F, t81651: F, t22893: F, t23164: F, t25320: F, t7521: F, t81632: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87536, t87545, t87547, t87566, t87582, t87584, t87586) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2050::<F>(t87535, t23185, t4283, t81914, t25300, t81591, t25303, t6579, t23110, t4292, t25288, t234, t4265);
        let (t87602, t87604, t87613, t87619, t87635) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2051::<F>(t23110, t23185, t25237, t23168, t25307, t25287, t81651, t22893, t23164, t25320, t7521, t81632);
    (t87536, t87545, t87547, t87566, t87582, t87584, t87586, t87602, t87604, t87613, t87619, t87635)
}
