//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1361;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1362;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1363;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta249<F: Float>(t10544: F, t1043: F, t676: F, t248: F, t884: F, t1041: F, t10478: F, t3128: F, t10472: F, t1015: F) -> (F, F, F, F, F, F, F, F) {
        let (t10832, t10868) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1361::<F>(t10544, t1043, t676);
        let (t10870, t10871, t10875, t10876) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1362::<F>(t10868, t248, t884, t1041, t10478, t3128, t10472);
        let (t10882, t10883) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1363::<F>(t1015, t10478, t10472);
    (t10832, t10868, t10870, t10871, t10875, t10876, t10882, t10883)
}
