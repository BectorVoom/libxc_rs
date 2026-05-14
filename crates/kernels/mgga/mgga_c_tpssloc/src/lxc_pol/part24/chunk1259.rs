//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1259/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1259<F: Float>(t82879: F, t82932: F, t82979: F, t83029: F, t83081: F, t83129: F, t83171: F, t83223: F, t25511: F, t6743: F, t49975: F, t6800: F, t23592: F, t23631: F, t974: F, t25721: F) -> (F, F, F, F, F) {
    let t83226 = t82879 + t82932 + t82979 + t83029 + t83081 + t83129 + t83171 + t83223;
    let t83233 = t6743 * t25511;
    let t83234 = t49975 * t6800;
    let t83239 = t23631 * t974 * t23592;
    let t83240 = t6743 * t25721;
    (t83226, t83233, t83234, t83239, t83240)
}
