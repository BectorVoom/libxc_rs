//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta466<F: Float>(t1235: F, t7284: F, t1240: F, t1251: F, t2122: F, t1170: F, t7295: F, t2121: F, t461: F, t6729: F, t7324: F, t2131: F, t23508: F) -> (F, F, F, F, F, F, F, F) {
        let (t24633, t24637, t24638, t24645, t24646, t24649, t24650, t24658) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1756::<F>(t1235, t7284, t1240, t1251, t2122, t1170, t7295, t2121, t461, t6729, t7324, t2131, t23508);
    (t24633, t24637, t24638, t24645, t24646, t24649, t24650, t24658)
}
