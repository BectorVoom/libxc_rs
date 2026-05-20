//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta648<F: Float>(t19573: F, t588: F, t592: F, t118: F, t2375: F, t6320: F, t12300: F, t6422: F, t12365: F, t1358: F, t19836: F, t12250: F, t6387: F) -> (F, F, F, F, F, F, F) {
        let (t57227, t57229, t57235, t57308, t57310, t57324, t57342) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2188::<F>(t19573, t588, t592, t118, t2375, t6320, t12300, t6422, t12365, t1358, t19836, t12250, t6387);
    (t57227, t57229, t57235, t57308, t57310, t57324, t57342)
}
