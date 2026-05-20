//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta531<F: Float>(t1720: F, t7348: F, t1190: F, t8054: F, t1751: F, t7284: F, t7287: F, t1251: F, t1409: F, t24602: F, t24601: F, t1090: F, t27381: F) -> (F, F, F, F, F, F, F) {
        let (t27422, t27424, t27426, t27427, t27433, t27434, t27437) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1868::<F>(t1720, t7348, t1190, t8054, t1751, t7284, t7287, t1251, t1409, t24602, t24601, t1090, t27381);
    (t27422, t27424, t27426, t27427, t27433, t27434, t27437)
}
