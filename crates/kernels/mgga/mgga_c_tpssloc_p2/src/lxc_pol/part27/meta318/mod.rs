//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta318<F: Float>(t3399: F, t445: F, t1143: F, t3375: F, t1124: F, t3331: F, t11282: F, t440: F, t11135: F, t11203: F, t1127: F, t3355: F) -> (F, F, F, F, F, F, F) {
        let (t11292, t11297, t11303, t11310, t11314, t11317, t11349) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1391::<F>(t3399, t445, t1143, t3375, t1124, t3331, t11282, t440, t11135, t11203, t1127, t3355);
    (t11292, t11297, t11303, t11310, t11314, t11317, t11349)
}
