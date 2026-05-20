//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta195<F: Float>(t1557: F, t2787: F, t912: F, t2792: F, t1547: F, t2798: F, t896: F, t2766: F, t2802: F, t4335: F, t4340: F, t4345: F, t4349: F) -> (F, F, F, F, F, F) {
        let (t4358, t4359, t4361, t4362, t4363, t4370) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk937::<F>(t1557, t2787, t912, t2792, t1547, t2798, t896, t2766, t2802, t4335, t4340, t4345, t4349);
    (t4358, t4359, t4361, t4362, t4363, t4370)
}
