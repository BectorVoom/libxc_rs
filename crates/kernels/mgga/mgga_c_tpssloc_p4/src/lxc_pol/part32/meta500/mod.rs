//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta500<F: Float>(t1410: F, t2240: F, t4017: F, t71: F, t12568: F, t33: F, t3953: F, t608: F, t1437: F, t641: F, t72: F, t4021: F, t79: F, t2235: F, t3961: F, t605: F, t3967: F, t1433: F, t645: F, t12725: F, t1873: F, t19456: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26016, t26024, t26028, t26055, t26063, t26066) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1819::<F>(t1410, t2240, t4017, t71, t12568, t33, t3953, t608, t1437, t641, t72, t4021, t79);
        let (t26067, t26070, t26073, t26076, t26090, t26109, t26111) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1820::<F>(t26066, t72, t1410, t2235, t3961, t605, t3967, t1433, t645, t12725, t1873, t19456);
    (t26016, t26024, t26028, t26055, t26063, t26067, t26070, t26073, t26076, t26090, t26109, t26111)
}
