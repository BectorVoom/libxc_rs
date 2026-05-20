//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta310<F: Float>(t1041: F, t10870: F, t3048: F, t3053: F, t10478: F, t3128: F, t10472: F, t1015: F, t1030: F, t3036: F, t3033: F, t248: F, t3041: F, t3101: F) -> (F, F, F, F, F, F, F) {
        let (t10871, t10873, t10876, t10883, t10889, t10891, t10895) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1377::<F>(t1041, t10870, t3048, t3053, t10478, t3128, t10472, t1015, t1030, t3036, t3033, t248, t3041, t3101);
    (t10871, t10873, t10876, t10883, t10889, t10891, t10895)
}
