//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1044/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1044<F: Float>(t581: F, t949: F, t3753: F, t2741: F, t3748: F, t8523: F, t3950: F, t837: F, t2703: F, t2724: F, t3932: F, t3931: F, t2723: F, t8561: F, t2725: F, t2459: F, t969: F) -> (F, F, F, F, F, F, F) {
    let t11592 = t949 * t581;
    let t11593 = t3753 * t11592;
    let t11594 = t2741 * t11593;
    let t11597 = t3748 * t11592;
    let t11598 = t8523 * t11597;
    let t11601 = t3950 * t837;
    let t11602 = t2741 * t11601;
    let t11607 = t2724 * t2703;
    let t11608 = t3932 * t11607;
    let t11609 = t3931 * t11608;
    let t11612 = t8561 * t2723;
    let t11613 = t3932 * t11612;
    let t11614 = t3931 * t11613;
    let t11617 = t3932 * t2725;
    let t11618 = t3931 * t11617;
    let t11621 = t969 * t2459;
    (t11594, t11598, t11602, t11609, t11614, t11618, t11621)
}
