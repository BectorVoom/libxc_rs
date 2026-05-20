//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta594<F: Float>(t19211: F, t225: F, t19253: F, t19121: F, t19259: F, t112: F, t20148: F, t5544: F, t868: F, t5527: F, t1484: F, t4303: F) -> (F, F, F, F, F, F, F, F) {
        let (t65208, t66822, t66845, t66860, t66958, t67123, t67128, t67164) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1982::<F>(t19211, t225, t19253, t19121, t19259, t112, t20148, t5544, t868, t5527, t1484, t4303);
    (t65208, t66822, t66845, t66860, t66958, t67123, t67128, t67164)
}
