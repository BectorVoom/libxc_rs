//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1336;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta237<F: Float>(t1891: F, t67: F, t246: F, t232: F, t2379: F, t2646: F, t2645: F, t2647: F, t9626: F, t210: F, t2553: F, t804: F, t2631: F, t828: F) -> (F, F, F, F, F, F, F, F) {
        let (t9645, t9646, t9647, t9649, t9653, t9657) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1336::<F>(t1891, t67, t246, t232, t2379, t2646, t2645, t2647, t9626, t210, t2553, t804);
        let (t9660, t9661) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1337::<F>(t2631, t828, t232);
    (t9645, t9646, t9647, t9649, t9653, t9657, t9660, t9661)
}
