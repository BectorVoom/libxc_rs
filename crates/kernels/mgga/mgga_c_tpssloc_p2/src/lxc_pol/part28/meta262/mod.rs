//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta262<F: Float>(t1499: F, t1898: F, t249: F, t1512: F, t6614: F, t1516: F, t6621: F, t1484: F, t6638: F, t6637: F, t6552: F, t232: F, t4282: F) -> (F, F, F, F, F, F, F, F) {
        let (t7503, t7504, t7506, t7508, t7520, t7521, t7522, t7524) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1130::<F>(t1499, t1898, t249, t1512, t6614, t1516, t6621, t1484, t6638, t6637, t6552, t232, t4282);
    (t7503, t7504, t7506, t7508, t7520, t7521, t7522, t7524)
}
