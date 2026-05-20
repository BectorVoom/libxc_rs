//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta445<F: Float>(t252: F, t776: F, t829: F, t6646: F, t22986: F, t6624: F, t814: F, t2627: F, t6604: F) -> (F, F, F, F, F, F) {
        let (t22988, t22989, t22990, t22992, t22993, t22996) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1780::<F>(t252, t776, t829, t6646, t22986, t6624, t814, t2627, t6604);
    (t22988, t22989, t22990, t22992, t22993, t22996)
}
