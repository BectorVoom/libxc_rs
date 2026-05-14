//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 821/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk821<F: Float>(t10544: F, t2840: F, t891: F, t275: F, t2843: F, t290: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F, t2885: F, t2884: F, t307: F, t302: F) -> (F, F, F, F, F, F, F, F) {
    let t10676 = 0.93011851851851851854e0 * t10544;
    let t10701 = 1.0 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0 / t2843 / t290;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    let t10765 = t919 * t2885;
    let t10770 = 1.0 / t2884 / t307;
    let t10771 = t302 * t10770;
    (t10676, t10702, t10704, t10740, t10747, t10756, t10765, t10771)
}
