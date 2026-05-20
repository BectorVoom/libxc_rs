//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta659<F: Float>(t91394: F, t91398: F, t91078: F, t91081: F, t91531: F, t91548: F, t1751: F, t7319: F, t1240: F, t5088: F, t11153: F, t497: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t93757, t93760, t93795, t93796, t93899, t93906, t94297, t94319, t94349) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2088::<F>(t91394, t91398, t91078, t91081, t91531, t91548, t1751, t7319, t1240, t5088, t11153, t497);
    (t93757, t93760, t93795, t93796, t93899, t93906, t94297, t94319, t94349)
}
