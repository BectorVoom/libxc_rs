//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 500/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk500<F: Float>(t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t94: F, t102: F, t177: F, t738: F, t745: F) -> (F, F, F, F, F, F, F) {
    let t2327 = 11.0 / 9.0 * t2281 * t107;
    let t2328 = t626 * t667;
    let t2331 = 1.0 / t655 / t106;
    let t2341 = 1.0 / t94;
    let t2349 = 1.0 / t102;
    let t2367 = t738 * t177;
    let t2368 = 1.0 / t2367;
    let t2369 = t745 * t745;
    (t2327, t2328, t2331, t2341, t2349, t2368, t2369)
}
