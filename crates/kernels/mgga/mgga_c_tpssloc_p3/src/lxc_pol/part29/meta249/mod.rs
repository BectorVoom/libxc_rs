//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1166;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1167;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta249<F: Float>(t28: F, t776: F, t868: F, t1081: F, t1877: F, t1915: F, t2522: F, t6666: F, t6670: F, t1873: F, t2314: F, t5113: F, t1268: F, t6534: F, t1271: F, t191: F, t192: F, t2020: F, t2018: F, t532: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6841, t6848, t6855, t6867, t6869) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1166::<F>(t28, t776, t868, t1081, t1877, t1915, t2522, t6666, t6670, t1873, t2314, t5113);
        let (t6871, t6875, t6876) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1167::<F>(t1268, t6534, t1271, t191, t192);
        let (t6877, t6878) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1168::<F>(t2020, t6876, t2018, t532);
    (t6841, t6848, t6855, t6867, t6869, t6871, t6875, t6876, t6877, t6878)
}
