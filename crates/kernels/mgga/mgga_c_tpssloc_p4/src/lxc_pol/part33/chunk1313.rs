//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1313/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1313<F: Float>(t25146: F, t5619: F, t5587: F, t87218: F, t25068: F, t5628: F, t20908: F, t6621: F, t1516: F, t98832: F, t5624: F, t232: F, t6605: F, t68025: F, t815: F) -> (F, F, F, F, F, F, F) {
    let t105309 = t25146 * t5619;
    let t105311 = t87218 * t5587;
    let t105313 = t25068 * t5628;
    let t105315 = t6621 * t20908;
    let t105317 = t98832 * t1516;
    let t105319 = t25068 * t5624;
    let t105325 = t6605 * t815 * t68025 * t232;
    (t105309, t105311, t105313, t105315, t105317, t105319, t105325)
}
