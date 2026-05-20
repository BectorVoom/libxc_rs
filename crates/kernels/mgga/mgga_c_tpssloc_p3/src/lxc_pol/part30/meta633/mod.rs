//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2040;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta633<F: Float>(t1887: F, t81959: F, t22690: F, t23171: F, t25319: F, t23143: F, t7525: F, t25238: F, t6579: F, t22893: F, t23164: F, t25312: F, t25273: F, t244: F, t268: F, t6559: F, t25250: F, t87202: F, t25316: F, t82038: F, t23110: F, t23185: F, t25272: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87642, t87653, t87666, t87669, t87679) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2040::<F>(t1887, t81959, t22690, t23171, t25319, t23143, t7525, t25238, t6579, t22893, t23164, t25312);
        let (t87680, t87710, t87712, t87714, t87718, t87729) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2041::<F>(t87679, t25273, t6579, t244, t268, t6559, t25250, t87202, t25316, t82038, t23110, t23185, t25272);
    (t87642, t87653, t87666, t87669, t87680, t87710, t87712, t87714, t87718, t87729)
}
