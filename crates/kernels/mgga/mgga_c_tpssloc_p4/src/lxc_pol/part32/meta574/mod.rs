//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta574<F: Float>(t7685: F, t7754: F, t19596: F, t2019: F, t1983: F, t7458: F, t7468: F, t1873: F, t6287: F, t652: F, t20162: F, t16524: F, t7769: F) -> (F, F, F, F, F, F, F, F) {
        let (t28843, t28860, t28861, t28863, t28864, t28866, t28888, t28890) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1949::<F>(t7685, t7754, t19596, t2019, t1983, t7458, t7468, t1873, t6287, t652, t20162, t16524, t7769);
    (t28843, t28860, t28861, t28863, t28864, t28866, t28888, t28890)
}
