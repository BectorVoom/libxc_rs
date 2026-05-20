//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta541<F: Float>(t4993: F, t7345: F, t5040: F, t7310: F, t27607: F, t460: F, t24682: F, t24658: F, t3: F, t24719: F, t3030: F, t1734: F, t3503: F) -> (F, F, F, F, F, F, F, F) {
        let (t27622, t27626, t27628, t27629, t27634, t27635, t27636, t27637) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1886::<F>(t4993, t7345, t5040, t7310, t27607, t460, t24682, t24658, t3, t24719, t3030, t1734, t3503);
    (t27622, t27626, t27628, t27629, t27634, t27635, t27636, t27637)
}
