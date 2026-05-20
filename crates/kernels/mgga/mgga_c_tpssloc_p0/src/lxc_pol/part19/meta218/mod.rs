//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta218<F: Float>(t10709: F, t959: F, t2904: F, t2925: F, t950: F, t2880: F, t2888: F, t931: F, t2924: F, t952: F, t2932: F, t2836: F, t914: F) -> (F, F, F, F, F, F, F, F) {
        let (t10711, t10713, t10715, t10717, t10720, t10723, t10724, t10727) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk916::<F>(t10709, t959, t2904, t2925, t950, t2880, t2888, t931, t2924, t952, t2932, t2836, t914);
    (t10711, t10713, t10715, t10717, t10720, t10723, t10724, t10727)
}
