//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk544;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk545;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta96<F: Float>(t2793: F, t913: F, t2792: F, t273: F, t276: F, t896: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F) -> (F, F, F, F, F, F) {
        let (t2794, t2796, t2798, t2799) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk544::<F>(t2793, t913, t2792, t273, t276, t896);
        let (t2800, t2807) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk545::<F>(t2798, t2799, t2764, t2766, t2773, t2778, t2782);
    (t2794, t2796, t2798, t2799, t2800, t2807)
}
