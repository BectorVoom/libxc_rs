//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta666<F: Float>(t91179: F, t16060: F, t6951: F, t1878: F, t80730: F, t6604: F, t80893: F, t6925: F, t6976: F, t26271: F, t80779: F, t22844: F) -> (F, F, F, F, F, F, F) {
        let (t91180, t91191, t91194, t91198, t91202, t91206, t91208) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2092::<F>(t91179, t16060, t6951, t1878, t80730, t6604, t80893, t6925, t6976, t26271, t80779, t22844);
    (t91180, t91191, t91194, t91198, t91202, t91206, t91208)
}
