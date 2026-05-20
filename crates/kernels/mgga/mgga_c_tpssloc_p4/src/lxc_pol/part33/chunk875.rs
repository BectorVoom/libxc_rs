//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 875/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk875<F: Float>(t16673: F, t816: F, t13278: F, t1512: F, t5587: F, t9667: F, t120: F, t5611: F, t2639: F, t5619: F, t5614: F, t2697: F, t5628: F) -> (F, F, F, F, F, F, F) {
    let t16872 = t16673 * t816;
    let t16877 = t13278 * t1512;
    let t16879 = t9667 * t5587;
    let t16891 = t120 * t5611;
    let t16940 = t2639 * t5619;
    let t16942 = t2639 * t5614;
    let t16954 = t2697 * t5628;
    (t16872, t16877, t16879, t16891, t16940, t16942, t16954)
}
