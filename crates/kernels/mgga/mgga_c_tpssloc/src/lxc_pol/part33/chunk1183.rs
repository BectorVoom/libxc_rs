//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1183/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1183<F: Float>(t1599: F, t25784: F, t225: F, t387: F, t5914: F, t345: F, t5943: F, t6705: F, t6704: F, t1634: F, t7624: F, t3174: F) -> (F, F, F, F, F, F) {
    let t28470 = t1599 * t25784;
    let t28474 = t5914 * t225 * t387;
    let t28475 = t345 * t28474;
    let t28480 = t6705 * t5943;
    let t28481 = t6704 * t28480;
    let t28484 = t7624 * t1634;
    let t28485 = t3174 * t28484;
    (t28470, t28474, t28475, t28480, t28481, t28485)
}
