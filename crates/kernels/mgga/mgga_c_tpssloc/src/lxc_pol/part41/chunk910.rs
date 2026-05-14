//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 910/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk910<F: Float>(t1011: F, t14205: F, t1019: F, t1615: F, t3131: F, t1022: F, t360: F, t883: F, t13566: F, t13602: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14206 = t14205 * t1011;
    let t14207 = t14206 * t1019;
    let t14211 = t1615 * t3131;
    let t14218 = t1615 * t1022;
    let t14219 = t360 * t883;
    let t14245 = 0.23744444444444444444e-1 * t13566;
    let t14246 = 0.11872222222222222222e-1 * t13602;
    let t14263 = t1573 * t2904;
    let t14266 = t4408 * t923;
    let t14271 = t1561 * t2885;
    (t14207, t14211, t14218, t14219, t14245, t14246, t14263, t14266, t14271)
}
