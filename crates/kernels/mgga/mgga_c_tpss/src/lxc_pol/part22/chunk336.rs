//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 336/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk336<F: Float>(t1101: F, t581: F, t926: F, t451: F, t453: F) -> (F, F, F, F, F) {
    let t1102 = t1101 * t581;
    let t1103 = t926 * t1102;
    let t1106 = t451 * t451;
    let t1107 = 1.0 / t1106;
    let t1108 = t1107 * t453;
    (t1102, t1103, t1106, t1107, t1108)
}
