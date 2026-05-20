//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 698/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk698<F: Float>(t3611: F, t3612: F, t1215: F, t1235: F, t1246: F, t3493: F, t491: F, t1209: F, t3032: F, t3499: F) -> (F, F, F, F, F, F) {
    let t3613 = t3611 * t3612;
    let t3616 = t1235 * t1215;
    let t3617 = t3616 * t1246;
    let t3620 = t491 * t3493;
    let t3621 = t3620 * t1246;
    let t3623 = t3032 * t1209;
    let t3624 = t3499 * t3623;
    (t3613, t3617, t3620, t3621, t3623, t3624)
}
