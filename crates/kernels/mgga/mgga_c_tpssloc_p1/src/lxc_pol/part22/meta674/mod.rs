//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta674<F: Float>(t13822: F, t17777: F, t973: F, t2986: F, t4514: F, t48019: F, t48046: F, t10236: F, t17691: F, t13779: F, t17183: F, t16558: F, t2989: F, t10224: F, t5828: F, t42875: F, t5817: F, t17763: F, t2960: F, t18057: F, t225: F, t18059: F, t1020: F, t17960: F, t248: F, t3101: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61472, t61489, t61495, t61528, t61557, t61589) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2231::<F>(t13822, t17777, t973, t2986, t4514, t48019, t48046, t10236, t17691, t13779, t17183, t16558, t2989);
        let (t61597, t61600, t61602, t61621, t61646, t61655) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232::<F>(t10224, t5828, t973, t42875, t5817, t17763, t2960, t18057, t225, t18059, t1020, t17960, t248, t3101);
    (t61472, t61489, t61495, t61528, t61557, t61589, t61597, t61600, t61602, t61621, t61646, t61655)
}
