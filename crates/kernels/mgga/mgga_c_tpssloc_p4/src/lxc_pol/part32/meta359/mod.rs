//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta359<F: Float>(t225: F, t4943: F, t1720: F, t3030: F, t3609: F, t1009: F, t4940: F, t1243: F, t14704: F, t14710: F, t14720: F, t14781: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14980, t15026, t15027, t15031, t15032, t15072, t15074, t15083, t15094) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1409::<F>(t225, t4943, t1720, t3030, t3609, t1009, t4940, t1243, t14704, t14710, t14720, t14781);
    (t14980, t15026, t15027, t15031, t15032, t15072, t15074, t15083, t15094)
}
