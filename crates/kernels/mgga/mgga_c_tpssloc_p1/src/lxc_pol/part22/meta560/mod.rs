//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta560<F: Float>(t41654: F, t270: F, t276: F, t39267: F, t273: F, t242: F, t281: F, t283: F, t10770: F, t919: F, t10701: F, t888: F) -> (F, F, F, F, F, F, F, F) {
        let (t41904, t41935, t41942, t41959, t41961, t41962, t41984, t42023) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2064::<F>(t41654, t270, t276, t39267, t273, t242, t281, t283, t10770, t919, t10701, t888);
    (t41904, t41935, t41942, t41959, t41961, t41962, t41984, t42023)
}
