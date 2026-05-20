//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta651<F: Float>(t5107: F, t652: F, t6534: F, t22574: F, t56198: F, t8643: F, t26162: F, t57802: F, t22597: F, t7685: F, t2018: F, t3734: F, t1983: F, t7687: F, t26062: F, t645: F, t72: F, t26066: F, t2307: F, t7431: F, t1437: F, t6509: F, t1864: F, t4021: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90051, t90059, t90062, t90064, t90065) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2175::<F>(t5107, t652, t6534, t22574, t56198, t8643, t26162, t57802, t22597, t7685, t2018, t3734);
        let (t90068, t90072, t90076, t90080, t90090, t90094) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2176::<F>(t1983, t7687, t90065, t26062, t645, t72, t26066, t2307, t7431, t1437, t6509, t1864, t4021);
    (t90051, t90059, t90062, t90064, t90068, t90072, t90076, t90080, t90090, t90094)
}
