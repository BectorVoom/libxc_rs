//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk799;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta169<F: Float>(t252: F, t9584: F, t2591: F, t852: F, t225: F, t2711: F, t2594: F, t2690: F, t841: F, t812: F, t849: F, t2697: F, t2707: F, t241: F, t6589: F, t67: F, t820: F, t9458: F, t2613: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9585, t9587, t9590, t9593, t9600, t9601, t9602, t9604) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk799::<F>(t252, t9584, t2591, t852, t225, t2711, t2594, t2690, t841, t812, t849, t2697, t2707);
        let (t9607, t9609, t9612) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk800::<F>(t241, t6589, t67, t820, t9458, t2613, t68);
    (t9585, t9587, t9590, t9593, t9600, t9601, t9602, t9604, t9607, t9609, t9612)
}
