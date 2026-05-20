//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta229<F: Float>(t10431: F, t10513: F, t10929: F, t11005: F, t349: F, t225: F, t3167: F, t3166: F, t990: F, t10358: F, t381: F, t1049: F, t3020: F) -> (F, F, F, F, F, F) {
        let (t11007, t11008, t11010, t11013, t11016, t11018) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk936::<F>(t10431, t10513, t10929, t11005, t349, t225, t3167, t3166, t990, t10358, t381, t1049, t3020);
    (t11007, t11008, t11010, t11013, t11016, t11018)
}
