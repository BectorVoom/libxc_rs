//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta528<F: Float>(t12300: F, t3853: F, t12305: F, t3866: F, t12238: F, t68: F, t1340: F, t10021: F, t1336: F, t1339: F, t1354: F, t12365: F, t3858: F) -> (F, F, F, F, F, F, F) {
        let (t40114, t40116, t40118, t40119, t40123, t40124, t40126) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2062::<F>(t12300, t3853, t12305, t3866, t12238, t68, t1340, t10021, t1336, t1339, t1354, t12365, t3858);
    (t40114, t40116, t40118, t40119, t40123, t40124, t40126)
}
