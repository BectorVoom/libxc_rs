//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta573<F: Float>(t22595: F, t28826: F, t1983: F, t1799: F, t1845: F, t8643: F, t22574: F, t1390: F, t6347: F, t6878: F, t7685: F, t7688: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28827, t28829, t28830, t28831, t28833, t28834, t28835, t28837, t28841) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1948::<F>(t22595, t28826, t1983, t1799, t1845, t8643, t22574, t1390, t6347, t6878, t7685, t7688);
    (t28827, t28829, t28830, t28831, t28833, t28834, t28835, t28837, t28841)
}
