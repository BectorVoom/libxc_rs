//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta230<F: Float>(t1049: F, t3040: F, t3188: F, t10857: F, t381: F, t1060: F, t1022: F, t3166: F, t10947: F, t3185: F, t3199: F, t3196: F, t4684: F) -> (F, F, F, F, F, F, F, F) {
        let (t11023, t11024, t11027, t11028, t11031, t11034, t11037, t11040) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk937::<F>(t1049, t3040, t3188, t10857, t381, t1060, t1022, t3166, t10947, t3185, t3199, t3196, t4684);
    (t11023, t11024, t11027, t11028, t11031, t11034, t11037, t11040)
}
