//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta291<F: Float>(t10027: F, t222: F, t805: F, t9541: F, t2627: F, t852: F, t856: F, t68: F, t261: F, t2751: F) -> (F, F, F, F, F, F, F) {
        let (t10029, t10036, t10054, t10108, t10109, t10110, t10143) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1177::<F>(t10027, t222, t805, t9541, t2627, t852, t856, t68, t261, t2751);
    (t10029, t10036, t10054, t10108, t10109, t10110, t10143)
}
