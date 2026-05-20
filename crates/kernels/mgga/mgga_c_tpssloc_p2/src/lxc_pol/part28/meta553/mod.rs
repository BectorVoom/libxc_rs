//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta553<F: Float>(t2031: F, t83718: F, t2240: F, t240: F, t33: F, t6492: F, t2244: F, t63: F, t23993: F, t6495: F, t1860: F, t22489: F, t7031: F, t1864: F, t67: F, t835: F, t22534: F, t7032: F, t6486: F, t24165: F, t532: F, t80743: F, t81281: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t84237, t84241, t84242, t84245, t84248, t84270) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823::<F>(t2031, t83718, t2240, t240, t33, t6492, t2244, t63, t23993, t6495, t1860, t22489, t7031);
        let (t84280, t84283, t84285, t84347, t84400, t84423) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1824::<F>(t1860, t1864, t67, t835, t22534, t7032, t23993, t6486, t24165, t532, t80743, t81281);
    (t84237, t84241, t84242, t84245, t84248, t84270, t84280, t84283, t84285, t84347, t84400, t84423)
}
