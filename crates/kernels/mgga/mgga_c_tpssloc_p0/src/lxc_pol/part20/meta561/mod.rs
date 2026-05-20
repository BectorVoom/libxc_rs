//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2117;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2118;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta561<F: Float>(t10957: F, t3053: F, t271: F, t2770: F, t10321: F, t1041: F, t248: F, t3051: F, t10459: F, t3117: F, t10469: F, t990: F, t10471: F, t10875: F, t10468: F, t191: F, t349: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42303, t42308, t42322, t42324, t42332) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2117::<F>(t10957, t3053, t271, t2770, t10321, t1041, t248, t3051, t10459, t3117, t10469, t990);
        let (t42333, t42334, t42339, t42340) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2118::<F>(t10471, t42332, t10875, t10468, t191, t349);
        let t42341 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2119::<F>(t10471, t68);
    (t42303, t42308, t42322, t42324, t42332, t42333, t42334, t42339, t42340, t42341)
}
