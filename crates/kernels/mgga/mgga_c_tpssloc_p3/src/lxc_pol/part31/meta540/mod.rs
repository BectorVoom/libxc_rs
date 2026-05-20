//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1758;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta540<F: Float>(t22724: F, t22727: F, t22894: F, t80670: F, t154: F, t9533: F, t131: F, t3748: F, t2009: F, t9537: F, t22642: F, t22690: F, t22881: F, t2690: F, t22691: F, t1887: F, t22797: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81076, t81080, t81142, t81144, t81146, t81149) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1758::<F>(t22724, t22727, t22894, t80670, t154, t9533, t131, t3748, t2009, t9537, t22642, t22690, t22881);
        let (t81151, t81152, t81153, t81159) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1759::<F>(t154, t2690, t3748, t22691, t1887, t22797);
    (t81076, t81080, t81142, t81144, t81146, t81149, t81151, t81152, t81153, t81159)
}
