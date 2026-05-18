//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1301/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1301<F: Float>(t23384: F, t28663: F, t23511: F, t5928: F, t28638: F, t23665: F, t28605: F, t5932: F, t6743: F, t28653: F, t82822: F, t5936: F) -> (F, F, F, F, F, F, F) {
    let t100163 = t23384 * t28663;
    let t100165 = t23511 * t5928;
    let t100189 = t23384 * t28638;
    let t100193 = t23665 * t28605;
    let t100204 = t6743 * t5932;
    let t100215 = t82822 * t28653;
    let t100231 = t6743 * t5936;
    (t100163, t100165, t100189, t100193, t100204, t100215, t100231)
}
