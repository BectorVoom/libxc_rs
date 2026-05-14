//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 804/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk804<F: Float>(t1519: F, t213: F, t225: F, t794: F, t28: F, t40772: F, t1649: F, t2752: F, t1834: F, t22573: F, t7684: F, t2094: F, t40611: F, t193: F, t201: F, t7844: F) -> (F, F, F, F, F, F, F, F, F) {
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90544 = t794 * t1834;
    let t90566 = t213 * t1834 * t225;
    let t91655 = t7684 * t22573;
    let t92169 = t2094 * t40611;
    let t92319 = t193 * t201 * t7844;
    (t86873, t86893, t89953, t89992, t90544, t90566, t91655, t92169, t92319)
}
