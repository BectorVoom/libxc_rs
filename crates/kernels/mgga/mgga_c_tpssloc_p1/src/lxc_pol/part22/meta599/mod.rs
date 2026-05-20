//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta599<F: Float>(t1021: F, t820: F, t10375: F, t1612: F, t1041: F, t1539: F, t248: F, t42749: F, t47705: F, t47707: F, t47730: F, t10661: F, t1556: F) -> (F, F, F, F, F, F, F) {
        let (t48611, t48670, t48674, t48688, t48689, t48698, t48763) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2121::<F>(t1021, t820, t10375, t1612, t1041, t1539, t248, t42749, t47705, t47707, t47730, t10661, t1556);
    (t48611, t48670, t48674, t48688, t48689, t48698, t48763)
}
