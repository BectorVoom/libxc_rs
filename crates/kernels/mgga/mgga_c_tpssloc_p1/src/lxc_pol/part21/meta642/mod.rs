//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2432;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta642<F: Float>(t10469: F, t990: F, t10471: F, t10875: F, t10468: F, t191: F, t349: F, t68: F) -> (F, F, F, F, F, F) {
        let (t42332, t42333, t42334, t42339, t42340) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2432::<F>(t10469, t990, t10471, t10875, t10468, t191, t349);
        let t42341 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2433::<F>(t10471, t68);
    (t42332, t42333, t42334, t42339, t42340, t42341)
}
