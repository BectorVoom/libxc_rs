//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta406<F: Float>(t18232: F, t3297: F, t136: F, t1113: F, t18237: F, t18241: F, t11211: F, t11487: F, t14766: F, t15347: F, t15348: F, t15349: F, t18494: F, t18497: F, t18500: F, t18503: F, t18505: F, t18508: F, t18510: F, t18512: F) -> (F, F, F, F, F, F, F) {
        let (t18514, t18515, t18517, t18518, t18520, t18521, t18523) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1706::<F>(t18232, t3297, t136, t1113, t18237, t18241, t11211, t11487, t14766, t15347, t15348, t15349, t18494, t18497, t18500, t18503, t18505, t18508, t18510, t18512);
    (t18514, t18515, t18517, t18518, t18520, t18521, t18523)
}
