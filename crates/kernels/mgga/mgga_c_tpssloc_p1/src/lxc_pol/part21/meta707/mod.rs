//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta707<F: Float>(t10704: F, t4395: F, t2904: F, t4446: F, t10523: F, t1573: F, t10629: F, t1556: F, t2842: F, t10702: F, t10828: F, t1580: F) -> (F, F, F, F, F, F, F) {
        let (t49072, t49096, t49099, t49104, t49226, t49240, t49263) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2540::<F>(t10704, t4395, t2904, t4446, t10523, t1573, t10629, t1556, t2842, t10702, t10828, t1580);
    (t49072, t49096, t49099, t49104, t49226, t49240, t49263)
}
