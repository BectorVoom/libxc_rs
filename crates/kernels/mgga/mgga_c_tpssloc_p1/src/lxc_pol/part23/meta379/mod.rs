//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta379<F: Float>(t1592: F, t42891: F, t973: F, t10471: F, t47840: F, t10479: F, t10375: F, t1612: F, t1041: F, t1539: F, t248: F, t42749: F) -> (F, F, F, F, F) {
        let (t48397, t48569, t48570, t48670, t48674) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1181::<F>(t1592, t42891, t973, t10471, t47840, t10479, t10375, t1612, t1041, t1539, t248, t42749);
    (t48397, t48569, t48570, t48670, t48674)
}
