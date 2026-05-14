//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 850/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk850<F: Float>(t2840: F, t287: F, t275: F, t10294: F, t10544: F, t891: F, t2843: F, t290: F, t2924: F, t2932: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10660 = 1.0 / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = 0.36514074074074074075e0 * t10294;
    let t10676 = 0.93011851851851851854e0 * t10544;
    let t10701 = 1.0 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0 / t2843 / t290;
    let t10723 = t2924 * t2932;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    (t10661, t10675, t10676, t10702, t10704, t10723, t10740, t10747, t10756)
}
