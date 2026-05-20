//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1623;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta440<F: Float>(t23110: F, t6648: F, t23185: F, t225: F, t2717: F, t2719: F, t6553: F, t1880: F, t252: F, t794: F) -> (F, F, F, F, F, F) {
        let (t23186, t23187, t23196, t23197, t23198, t23204) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1623::<F>(t23110, t6648, t23185, t225, t2717, t2719, t6553, t1880, t252, t794);
    (t23186, t23187, t23196, t23197, t23198, t23204)
}
