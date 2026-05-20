//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta511<F: Float>(t1878: F, t22683: F, t221: F, t5308: F, t22844: F, t6604: F, t1361: F, t1339: F, t5287: F, t6936: F, t22779: F, t7712: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26284, t26285, t26286, t26288, t26289, t26290, t26292, t26293, t26295) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1838::<F>(t1878, t22683, t221, t5308, t22844, t6604, t1361, t1339, t5287, t6936, t22779, t7712);
    (t26284, t26285, t26286, t26288, t26289, t26290, t26292, t26293, t26295)
}
