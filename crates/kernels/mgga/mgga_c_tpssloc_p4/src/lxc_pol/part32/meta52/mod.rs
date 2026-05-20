//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk353;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta52<F: Float>(t607: F, t998: F, t974: F, t225: F, t990: F, t68: F, t369: F, t191: F, t349: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t999, t1000, t1003, t1004, t1005, t1008, t1009) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk353::<F>(t607, t998, t974, t225, t990, t68, t369, t191);
        let (t1010, t1011) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk354::<F>(t1009, t349, t68);
    (t999, t1000, t1003, t1004, t1005, t1008, t1009, t1010, t1011)
}
