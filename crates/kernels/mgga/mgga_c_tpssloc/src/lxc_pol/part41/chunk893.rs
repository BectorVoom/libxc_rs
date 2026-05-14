//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 893/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk893<F: Float>(t2427: F, t4101: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F, t1471: F, t32: F, t4095: F, t67: F, t758: F, t118: F, t1474: F, t2375: F, t4094: F, t706: F) -> (F, F, F, F, F, F, F, F) {
    let t13105 = 8.0 * t2427 * t4101;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    let t13115 = t32 * t1471;
    let t13119 = t4095 * t67;
    let t13121 = 0.36622894612013090108e-3 * t13119 * t758;
    let t13123 = t1474 * t118;
    let t13124 = t13123 * t2375;
    let t13133 = t706 * t4094;
    (t13105, t13107, t13109, t13113, t13115, t13121, t13124, t13133)
}
