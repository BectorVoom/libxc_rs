//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta679<F: Float>(t225: F, t27424: F, t27422: F, t24574: F, t27752: F, t27834: F, t3640: F, t11947: F, t8090: F, t27331: F, t9231: F, t46104: F, t7245: F) -> (F, F, F, F, F, F, F) {
        let (t95899, t95902, t95912, t95921, t95925, t95981, t96025) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2118::<F>(t225, t27424, t27422, t24574, t27752, t27834, t3640, t11947, t8090, t27331, t9231, t46104, t7245);
    (t95899, t95902, t95912, t95921, t95925, t95981, t96025)
}
