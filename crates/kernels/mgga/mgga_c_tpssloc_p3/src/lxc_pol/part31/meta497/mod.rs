//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta497<F: Float>(t28060: F, t6936: F, t6417: F, t6945: F, t1827: F, t26233: F, t1339: F, t6415: F, t22839: F, t6371: F, t1998: F, t236: F, t6330: F) -> (F, F, F, F, F, F, F) {
        let (t28061, t28063, t28065, t28067, t28068, t28070, t28073) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1692::<F>(t28060, t6936, t6417, t6945, t1827, t26233, t1339, t6415, t22839, t6371, t1998, t236, t6330);
    (t28061, t28063, t28065, t28067, t28068, t28070, t28073)
}
