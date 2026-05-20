//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta223<F: Float>(t41: F, t42: F, t53: F, t54: F, t2585: F, t2769: F, t73: F, t3241: F, t76: F, t107: F, t2281: F, t667: F) -> (F, F, F, F, F, F, F) {
        let (t9287, t9300, t9311, t9321, t9330, t9358, t9359) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1273::<F>(t41, t42, t53, t54, t2585, t2769, t73, t3241, t76, t107, t2281, t667);
    (t9287, t9300, t9311, t9321, t9330, t9358, t9359)
}
