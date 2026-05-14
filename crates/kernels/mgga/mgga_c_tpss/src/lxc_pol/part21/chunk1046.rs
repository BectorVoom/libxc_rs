//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1046/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1046<F: Float>(t1407: F, t2732: F, t2741: F, t11491: F, t3933: F, t3931: F, t1460: F, t672: F, t925: F, t140: F, t3927: F, t1465: F, t2465: F, t2725: F, t2682: F, t3941: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11630 = t1407 * t2732;
    let t11631 = t2741 * t11630;
    let t11636 = t11491 * t3933;
    let t11637 = t3931 * t11636;
    let t11640 = t672 * t1460;
    let t11641 = t925 * t11640;
    let t11645 = t140 * t3927;
    let t11647 = t925 * t11645 / 432.0;
    let t11648 = t1465 * t2465;
    let t11649 = t2741 * t11648;
    let t11652 = t1407 * t2725;
    let t11653 = t2741 * t11652;
    let t11659 = t2682 * t3941 / 432.0;
    (t11631, t11637, t11640, t11641, t11645, t11647, t11649, t11653, t11659)
}
