//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta689<F: Float>(t41115: F, t4191: F, t41107: F, t4166: F, t9670: F, t831: F, t12890: F, t751: F, t12932: F, t2427: F, t13133: F, t2430: F) -> (F, F, F, F, F, F, F) {
        let (t47079, t47081, t47092, t47093, t47160, t47163, t47165) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2504::<F>(t41115, t4191, t41107, t4166, t9670, t831, t12890, t751, t12932, t2427, t13133, t2430);
    (t47079, t47081, t47092, t47093, t47160, t47163, t47165)
}
