//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 900/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk900<F: Float>(t13602: F, t1553: F, t2403: F, t4392: F, t699: F, t13550: F, t13563: F, t1543: F, t2791: F, t2970: F, t4343: F, t973: F, t1036: F, t4617: F, t10422: F, t4574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13603 = 2.0 / 9.0 * t13602;
    let t13642 = t2403 * t1553;
    let t13644 = t699 * t4392;
    let t13645 = 0.10954222222222222222e0 * t13644;
    let t13650 = 0.19931111111111111111e0 * t13602;
    let t13675 = 0.22076e0 * t13550;
    let t13679 = 0.13418888888888888889e0 * t13563;
    let t13709 = 0.11038e0 * t13644;
    let t13712 = 0.20128333333333333334e0 * t13602;
    let t13727 = t1543 * t2791;
    let t13748 = t2970 * t4343;
    let t13750 = t973 * t13748 / 216.0;
    let t13758 = t4617 * t1036 / 2304.0;
    let t13765 = t10422 * t4574;
    (t13603, t13642, t13644, t13645, t13650, t13675, t13679, t13709, t13712, t13727, t13750, t13758, t13765)
}
