//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1396;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta350<F: Float>(t13969: F, t4599: F, t3039: F, t3069: F, t4669: F, t10231: F, t4338: F, t973: F, t4595: F, t3130: F, t3048: F, t4571: F, t3109: F, t4630: F, t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F, t1612: F, t3082: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13972, t13995, t14000, t14027, t14049) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1396::<F>(t13969, t4599, t3039, t3069, t4669, t10231, t4338, t973, t4595, t3130, t3048, t4571);
        let (t14059, t14077, t14080, t14084, t14085, t14117) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1397::<F>(t3109, t4630, t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1612, t3082);
    (t13972, t13995, t14000, t14027, t14049, t14059, t14077, t14080, t14084, t14085, t14117)
}
