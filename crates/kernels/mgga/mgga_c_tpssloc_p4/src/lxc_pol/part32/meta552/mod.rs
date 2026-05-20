//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1911;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta552<F: Float>(t27975: F, t72: F, t5392: F, t605: F, t5399: F, t1873: F, t19451: F, t1441: F, t1458: F, t109: F, t4028: F, t7467: F, t5493: F, t88: F, t7676: F, t22473: F, t5464: F, t5488: F, t6530: F, t22469: F, t27166: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27976, t27979, t27982, t28001, t28002) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1911::<F>(t27975, t72, t5392, t605, t5399, t1873, t19451, t1441, t1458);
        let (t28004, t28006, t28007, t28009, t28011, t28017) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1912::<F>(t109, t1873, t28002, t4028, t7467, t5493, t88, t7676, t22473, t5464, t5488, t6530, t22469, t27166);
    (t27976, t27979, t27982, t28001, t28002, t28004, t28006, t28007, t28009, t28011, t28017)
}
