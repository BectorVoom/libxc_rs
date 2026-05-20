//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta17 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk127;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk128;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk129;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk130;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta17<F: Float>(t273: F, t276: F, t279: F, t285: F, t275: F, t148: F, t154: F, t157: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t287, t290, t291) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk127::<F>(t273, t276, t279, t285);
        let (t293, t300) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk128::<F>(t275, t291, t148, t154, t157, zeta_threshold);
        let t302 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk129::<F>(t273);
        let (t307, t310, t311) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk130::<F>(t273, t276, t279, t285);
        let t315 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk131::<F>(t273);
    (t287, t290, t291, t293, t300, t302, t307, t310, t311, t315)
}
