//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta754<F: Float>(t12339: F, t5310: F, t16150: F, t3866: F, t16155: F, t1827: F, t40123: F, t1824: F, t3850: F, t16060: F, t3802: F, t1799: F) -> (F, F, F, F, F, F, F) {
        let (t54133, t54135, t54138, t54151, t54153, t54162, t54165) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2628::<F>(t12339, t5310, t16150, t3866, t16155, t1827, t40123, t1824, t3850, t16060, t3802, t1799);
    (t54133, t54135, t54138, t54151, t54153, t54162, t54165)
}
