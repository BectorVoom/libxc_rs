//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1292;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta229<F: Float>(t1891: F, t67: F, t246: F, t2628: F, t835: F, t812: F, t2690: F, t815: F) -> (F, F, F, F, F, F) {
        let (t9645, t9646, t9666, t9667, t9670, t9671) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1292::<F>(t1891, t67, t246, t2628, t835, t812, t2690, t815);
    (t9645, t9646, t9666, t9667, t9670, t9671)
}
