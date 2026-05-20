//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta641<F: Float>(t16662: F, t25: F, t28248: F, t776: F, t22960: F, t10143: F, t1408: F, t25374: F, t1530: F, t584: F, t86730: F, t5397: F, t868: F) -> (F, F, F, F, F, F) {
        let (t98050, t98058, t98059, t98065, t98069, t98075) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1909::<F>(t16662, t25, t28248, t776, t22960, t10143, t1408, t25374, t1530, t584, t86730, t5397, t868);
    (t98050, t98058, t98059, t98065, t98069, t98075)
}
