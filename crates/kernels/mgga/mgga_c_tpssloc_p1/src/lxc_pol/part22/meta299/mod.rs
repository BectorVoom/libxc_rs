//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1463;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta299<F: Float>(t10231: F, t4338: F, t973: F, t13969: F, t4595: F, t3130: F, t3048: F, t4571: F, t3109: F, t4630: F, t3108: F, t4640: F, t1611: F, t3047: F) -> (F, F, F, F, F, F, F) {
        let (t14000, t14025, t14027, t14049, t14059, t14077) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1463::<F>(t10231, t4338, t973, t13969, t4595, t3130, t3048, t4571, t3109, t4630, t3108, t4640);
        let t14080 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1464::<F>(t1611, t3047);
    (t14000, t14025, t14027, t14049, t14059, t14077, t14080)
}
