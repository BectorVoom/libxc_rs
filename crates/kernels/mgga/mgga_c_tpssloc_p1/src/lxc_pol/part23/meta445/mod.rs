//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta445<F: Float>(t118: F, t20416: F, t3739: F, t794: F, t16094: F, t16095: F, t6347: F, t686: F, t213: F, t20582: F, t40021: F, t20356: F, t40412: F) -> (F, F, F, F, F) {
        let (t74702, t74724, t74726, t74741, t74745) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1290::<F>(t118, t20416, t3739, t794, t16094, t16095, t6347, t686, t213, t20582, t40021, t20356, t40412);
    (t74702, t74724, t74726, t74741, t74745)
}
