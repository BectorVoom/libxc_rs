//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1210/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1210<F: Float>(t31280: F, t33185: F, t23877: F, t7467: F, t7769: F, t83980: F, t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t7015: F, t86647: F, t16524: F, t31285: F, t16521: F) -> (F, F, F, F, F, F, F, F) {
    let t120792 = 54.0 * t33185 * t31280;
    let t120793 = t23877 * t7467;
    let t120795 = t83980 * t7769;
    let t120800 = 27.0 * t20173 * t33193;
    let t120803 = 27.0 * t3941 * t8326 * t4072;
    let t120804 = t86647 * t7015;
    let t120807 = 27.0 * t16524 * t31285;
    let t120809 = 0.135e2 * t16521 * t8326;
    (t120792, t120793, t120795, t120800, t120803, t120804, t120807, t120809)
}
