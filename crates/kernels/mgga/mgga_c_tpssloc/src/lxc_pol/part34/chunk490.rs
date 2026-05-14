//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 490/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk490<F: Float>(t1788: F, t592: F, t546: F, t68: F, t1365: F, t1799: F, t1831: F, t3866: F, t1835: F, t225: F) -> (F, F, F, F, F) {
    let t5266 = t592 * t1788;
    let t5278 = t546 * t68;
    let t5279 = t1365 * t1799;
    let t5306 = t3866 * t1831;
    let t5321 = t1835 * t225;
    (t5266, t5278, t5279, t5306, t5321)
}
