//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta730<F: Float>(t21185: F, t41935: F, t896: F, t17210: F, t4370: F, t13629: F, t5705: F, t17271: F, t4362: F, t41942: F, t17218: F, t41962: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F) -> (F, F, F, F, F, F, F) {
        let (t68619, t68626, t68628, t68630, t68633, t68635, t68637) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2396::<F>(t21185, t41935, t896, t17210, t4370, t13629, t5705, t17271, t4362, t41942, t17218, t41962, t47787, t59700, t59702, t59704, t60274);
    (t68619, t68626, t68628, t68630, t68633, t68635, t68637)
}
