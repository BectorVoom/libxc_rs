//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta496<F: Float>(t25374: F, t25927: F, t1081: F, t1530: F, t28: F, t4303: F, t1649: F, t776: F, t868: F, t1877: F, t1915: F, t22959: F, t23290: F, t25013: F, t2522: F, t25354: F, t25358: F, t25372: F, t25397: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t6666: F, t6670: F, t6841: F, t6848: F, t7541: F, t7649: F, t7656: F) -> (F, F, F, F, F, F) {
        let (t25928, t25930, t25934, t25938, t25945, t25949) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1815::<F>(t25374, t25927, t1081, t1530, t28, t4303, t1649, t776, t868, t1877, t1915, t22959, t23290, t25013, t2522, t25354, t25358, t25372, t25397, t25892, t25898, t25901, t25905, t25921, t6666, t6670, t6841, t6848, t7541, t7649, t7656);
    (t25928, t25930, t25934, t25938, t25945, t25949)
}
