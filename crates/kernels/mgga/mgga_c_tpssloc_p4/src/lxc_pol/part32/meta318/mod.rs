//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta318<F: Float>(t11135: F, t11203: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t1143: F, t3400: F, t11292: F, t440: F, t1124: F, t3356: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11314, t11317, t11350, t11352, t11361, t11365, t11369, t11372, t11415) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1346::<F>(t11135, t11203, t1127, t3355, t427, t3358, t435, t1143, t3400, t11292, t440, t1124, t3356);
    (t11314, t11317, t11350, t11352, t11361, t11365, t11369, t11372, t11415)
}
