//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 906/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk906<F: Float>(t11647: F, t485: F, t3576: F, t3604: F, t3585: F, t820: F, t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t1190: F, t3030: F) -> (F, F, F, F, F, F, F) {
    let t11649 = t485 * t11647 / F::new(10368.0);
    let t11665 = t3604 * t3576;
    let t11668 = t820 * t3585;
    let t11677 = t3575 * t10401;
    let t11678 = t3610 * t11677;
    let t11692 = t3624 * t11677;
    let t11697 = t820 * t3521;
    let t11707 = t1190 * t3030;
    (t11649, t11665, t11668, t11678, t11692, t11697, t11707)
}
