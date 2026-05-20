//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1390;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta332<F: Float>(t11702: F, t1213: F, t3490: F, t3523: F, t1190: F, t3030: F, t3032: F, t3505: F, t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11703, t11705, t11707, t11708, t11709, t11712, t11713) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1390::<F>(t11702, t1213, t3490, t3523, t1190, t3030, t3032, t3505, t10469, t466, t10471);
        let (t11715, t11716, t11717) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1391::<F>(t1208, t478, t10477, t483);
    (t11703, t11705, t11707, t11708, t11709, t11712, t11713, t11715, t11716, t11717)
}
