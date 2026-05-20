//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta326<F: Float>(t1285: F, t2221: F, t1287: F, t522: F, t9216: F, t9218: F, t1294: F, t9713: F, t25: F, t526: F, t28: F, t528: F) -> (F, F, F, F, F, F, F) {
        let (t12050, t12052, t12054, t12057, t12059, t12061, t12072) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1257::<F>(t1285, t2221, t1287, t522, t9216, t9218, t1294, t9713, t25, t526, t28, t528);
    (t12050, t12052, t12054, t12057, t12059, t12061, t12072)
}
