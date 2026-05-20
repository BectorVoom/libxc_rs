//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta598<F: Float>(t3082: F, t4622: F, t41666: F, t43398: F, t10471: F, t47840: F, t10479: F, t10216: F, t13797: F, t3067: F, t353: F, t373: F, t383: F) -> (F, F, F, F, F, F) {
        let (t48431, t48496, t48569, t48570, t48585, t48607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2120::<F>(t3082, t4622, t41666, t43398, t10471, t47840, t10479, t10216, t13797, t3067, t353, t373, t383);
    (t48431, t48496, t48569, t48570, t48585, t48607)
}
