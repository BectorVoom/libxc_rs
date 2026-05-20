//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk657;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta104<F: Float>(t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F, t702: F, t683: F, t681: F, t125: F, t701: F, t141: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2405, t2406, t2408) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk657::<F>(t2388, t2391, t2394, t2398, t2400, t2403, t702, t683);
        let (t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk658::<F>(t681, t125, t701, t141);
    (t2405, t2406, t2408, t2409, t2410, t2411, t2412, t2413, t2414, t2415, t2417)
}
