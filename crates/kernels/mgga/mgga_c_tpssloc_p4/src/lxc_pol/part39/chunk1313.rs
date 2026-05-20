//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1313/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1313<F: Float>(t2281: F, t2331: F, t656: F, t30133: F, t576: F, t30094: F, t580: F, t2212: F, t3931: F, t1395: F, t8217: F, t2205: F, t3946: F) -> (F, F, F, F, F, F, F) {
    let t110140 = t2281 * t2331;
    let t110143 = t2281 * t656;
    let t110274 = t576 * t30133;
    let t110276 = t30094 * t580;
    let t110280 = t3931 * t2212;
    let t110282 = t1395 * t8217;
    let t110284 = t2205 * t3946;
    (t110140, t110143, t110274, t110276, t110280, t110282, t110284)
}
