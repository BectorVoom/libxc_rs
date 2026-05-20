//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk744;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta124<F: Float>(t2765: F, t2766: F, t2773: F, t2778: F, t2782: F, t291: F, t888: F, t892: F, t914: F, t287: F, t891: F, t275: F, t912: F) -> (F, F, F, F, F, F, F) {
        let (t2784, t2786, t2787, t2789, t2791, t2792) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk744::<F>(t2765, t2766, t2773, t2778, t2782, t291, t888, t892, t914, t287, t891, t275);
        let t2793 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk745::<F>(t912);
    (t2784, t2786, t2787, t2789, t2791, t2792, t2793)
}
