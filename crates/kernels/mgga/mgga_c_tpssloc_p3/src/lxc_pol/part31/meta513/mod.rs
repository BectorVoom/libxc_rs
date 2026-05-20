//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta513<F: Float>(t23097: F, t28396: F, t1516: F, t25068: F, t5624: F, t6621: F, t5572: F, t6581: F, t16758: F, t232: F, t6646: F, t1888: F) -> (F, F, F, F, F, F, F) {
        let (t28397, t28399, t28401, t28403, t28418, t28419, t28420) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1709::<F>(t23097, t28396, t1516, t25068, t5624, t6621, t5572, t6581, t16758, t232, t6646, t1888);
    (t28397, t28399, t28401, t28403, t28418, t28419, t28420)
}
