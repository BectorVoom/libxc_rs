//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta551<F: Float>(t1883: F, t82045: F, t23012: F, t6568: F, t23205: F, t82038: F, t1081: F, t2752: F, t608: F, t9239: F, t22573: F, t6875: F) -> (F, F, F, F, F, F) {
        let (t82218, t82259, t82294, t83555, t83717, t83886) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1779::<F>(t1883, t82045, t23012, t6568, t23205, t82038, t1081, t2752, t608, t9239, t22573, t6875);
    (t82218, t82259, t82294, t83555, t83717, t83886)
}
