//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1540;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1541;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta422<F: Float>(t22674: F, t6907: F, t6897: F, t131: F, t557: F, t209: F, t1878: F, t212: F, t225: F, t6968: F, t22642: F, t268: F, t534: F, t6559: F, t1338: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22675, t22676, t22683, t22684, t22685, t22690) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1540::<F>(t22674, t6907, t6897, t131, t557, t209, t1878, t212, t225);
        let (t22691, t22692, t22704) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1541::<F>(t22690, t6968, t22642, t268, t534, t6559);
        let t22705 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1542::<F>(t1338, t22690);
    (t22675, t22676, t22683, t22684, t22685, t22690, t22691, t22692, t22704, t22705)
}
