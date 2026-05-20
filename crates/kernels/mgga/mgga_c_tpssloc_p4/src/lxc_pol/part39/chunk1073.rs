//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1073/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1073<F: Float>(t4290: F, t808: F, t13380: F, t4182: F, t68: F, t9971: F, t226: F, t13263: F, t4282: F, t2633: F, t9632: F, t2732: F, t4234: F) -> (F, F, F, F, F, F, F) {
    let t13390 = t808 * t4290;
    let t13393 = t13380 * t4182;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13398 = t4282 * t13263;
    let t13401 = t4282 * t2633;
    let t13404 = t4282 * t9632;
    let t13407 = t2732 * t4234;
    (t13390, t13393, t13397, t13398, t13401, t13404, t13407)
}
