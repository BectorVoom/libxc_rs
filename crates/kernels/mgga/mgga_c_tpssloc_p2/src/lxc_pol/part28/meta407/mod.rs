//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta407<F: Float>(t22674: F, t6907: F, t6897: F, t131: F, t557: F, t209: F, t1878: F, t3734: F, t6890: F, t6889: F, t212: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22675, t22676, t22683, t22684, t22685, t22686, t22687, t22688, t22690) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1572::<F>(t22674, t6907, t6897, t131, t557, t209, t1878, t3734, t6890, t6889, t212, t225);
    (t22675, t22676, t22683, t22684, t22685, t22686, t22687, t22688, t22690)
}
