//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1848;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta477<F: Float>(t2771: F, t6690: F, t23593: F, t3034: F, t38: F, t131: F, t350: F, t3030: F, t344: F) -> (F, F, F, F, F, F, F) {
        let (t23594, t23595, t23598, t23599, t23600, t23601) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1848::<F>(t2771, t6690, t23593, t3034, t38, t131, t350);
        let t23602 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1849::<F>(t3030, t344);
    (t23594, t23595, t23598, t23599, t23600, t23601, t23602)
}
