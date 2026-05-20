//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3133/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3133<F: Float>(t4928: F, t1174: F, t135: F, t18525: F, t11583: F, t17691: F, t12652: F, t4723: F, t3428: F, t6109: F, t6146: F, t698: F) -> (F, F, F, F, F, F) {
    let t64851 = t4928 * t4928;
    let t64858 = t1174 * t135 * t18525;
    let t64870 = t11583 * t17691;
    let t64874 = t4723 * t12652;
    let t64878 = t6109 * t3428;
    let t64881 = t1174 * t698 * t6146;
    (t64851, t64858, t64870, t64874, t64878, t64881)
}
