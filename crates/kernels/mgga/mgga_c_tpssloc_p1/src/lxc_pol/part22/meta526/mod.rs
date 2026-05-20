//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta526<F: Float>(t13228: F, t828: F, t13223: F, t232: F, t253: F, t254: F, t1530: F, t776: F, t868: F, t1022: F, t1409: F, t382: F) -> (F, F, F, F, F, F, F) {
        let (t25093, t25115, t25168, t25365, t25374, t25548, t25757) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1996::<F>(t13228, t828, t13223, t232, t253, t254, t1530, t776, t868, t1022, t1409, t382);
    (t25093, t25115, t25168, t25365, t25374, t25548, t25757)
}
