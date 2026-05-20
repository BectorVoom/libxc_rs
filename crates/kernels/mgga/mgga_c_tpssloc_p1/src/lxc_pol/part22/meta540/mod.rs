//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta540<F: Float>(t3684: F, t39503: F, t677: F, t9722: F, t12110: F, t9888: F, t9467: F, t9919: F, t2393: F, t2535: F, t9882: F, t2420: F, t701: F, t9778: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39505, t39506, t39508, t39512, t39514, t39516, t39518, t39519, t39521, t39522, t39529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2026::<F>(t3684, t39503, t677, t9722, t12110, t9888, t9467, t9919, t2393, t2535, t9882, t2420, t701, t9778);
    (t39505, t39506, t39508, t39512, t39514, t39516, t39518, t39519, t39521, t39522, t39529)
}
