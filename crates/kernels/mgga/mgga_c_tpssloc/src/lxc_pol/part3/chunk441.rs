//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 441/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk441<F: Float>(t1484: F, t210: F, t214: F, t785: F, t787: F, t797: F, t252: F, t119: F, t225: F) -> (F, F, F, F, F, F) {
    let t1489 = t210 * t214 * t1484;
    let t1492 = -t785 - 0.16666666666666666666e-2 * t787 * t1489 - t797;
    let t1493 = t1492 * t252;
    let t1495 = t119 * t1484;
    let t1496 = t210 * t1495;
    let t1499 = t1492 * t225;
    (t1489, t1492, t1493, t1495, t1496, t1499)
}
