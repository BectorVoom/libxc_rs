//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta134 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk878;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta134<F: Float>(t40: F, t52: F, t4101: F, t707: F, t1409: F, t75: F, t3966: F, t607: F, t767: F, t78: F, t771: F, zeta_threshold: F, t1489: F, t2563: F, t131: F, t2570: F, t205: F) -> (F, F, F, F, F, F, F, F) {
        let (t4102, t4103, t4104, t4111, t4119) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk878::<F>(t40, t52, t4101, t707, t1409, t75, t3966, t607, t767, t78, t771, zeta_threshold);
        let (t4124, t4126, t4127) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk879::<F>(t1489, t2563, t131, t2570, t205);
    (t4102, t4103, t4104, t4111, t4119, t4124, t4126, t4127)
}
