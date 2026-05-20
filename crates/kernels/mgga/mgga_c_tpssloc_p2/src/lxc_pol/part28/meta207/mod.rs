//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta207<F: Float>(t225: F, t4552: F, t68: F, t369: F, t1031: F, t1611: F, t1036: F, t1612: F, t1616: F, t248: F, t3101: F, t1020: F) -> (F, F, F, F, F, F, F) {
        let (t4615, t4616, t4617, t4622, t4625, t4630, t4631) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk954::<F>(t225, t4552, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101, t1020);
    (t4615, t4616, t4617, t4622, t4625, t4630, t4631)
}
