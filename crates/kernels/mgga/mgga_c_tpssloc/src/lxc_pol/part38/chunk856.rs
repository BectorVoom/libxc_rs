//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 856/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk856<F: Float>(t225: F, t2594: F, t2690: F, t841: F, t812: F, t849: F, t2697: F, t2707: F, t241: F, t6589: F, t67: F, t2613: F, t68: F, t816: F, t2632: F, t2678: F) -> (F, F, F, F, F, F, F, F) {
    let t9593 = t2594 * t225;
    let t9600 = t841 * t2690;
    let t9601 = t812 * t9600;
    let t9602 = t9601 * t849;
    let t9604 = t2697 * t2707;
    let t9607 = t241 * t6589 * t67;
    let t9612 = t2613 * t68;
    let t9613 = t9612 * t816;
    let t9632 = t2632 * t2678;
    (t9593, t9601, t9602, t9604, t9607, t9612, t9613, t9632)
}
