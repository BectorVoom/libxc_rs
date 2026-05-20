//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta674<F: Float>(t90980: F, t90993: F, t91000: F, t91149: F, t91167: F, t91305: F, t91312: F, t91394: F, t91398: F, t91078: F, t91081: F, t91531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2103::<F>(t90980, t90993, t91000, t91149, t91167, t91305, t91312, t91394, t91398, t91078, t91081, t91531);
    (t93595, t93605, t93615, t93650, t93656, t93721, t93723, t93757, t93760, t93795, t93796, t93899)
}
