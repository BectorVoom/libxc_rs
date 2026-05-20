//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta641<F: Float>(t14137: F, t6765: F, t7583: F, t83138: F, t25644: F, t82926: F, t23512: F, t25486: F, t23519: F, t25492: F, t1597: F, t607: F) -> (F, F, F, F, F, F) {
        let (t88339, t88341, t88348, t88351, t88354, t88360) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2051::<F>(t14137, t6765, t7583, t83138, t25644, t82926, t23512, t25486, t23519, t25492, t1597, t607);
    (t88339, t88341, t88348, t88351, t88354, t88360)
}
