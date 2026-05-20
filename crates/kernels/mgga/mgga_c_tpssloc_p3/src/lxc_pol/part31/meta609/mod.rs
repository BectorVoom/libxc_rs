//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta609<F: Float>(t90963: F, t90970: F, t90983: F, t90987: F, t1338: F, t27051: F, t91010: F, t91113: F, t91120: F, t91135: F, t91137: F, t91140: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93590, t93592, t93599, t93600, t93607, t93618, t93633, t93636, t93644, t93645, t93646) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1854::<F>(t90963, t90970, t90983, t90987, t1338, t27051, t91010, t91113, t91120, t91135, t91137, t91140);
    (t93590, t93592, t93599, t93600, t93607, t93618, t93633, t93636, t93644, t93645, t93646)
}
