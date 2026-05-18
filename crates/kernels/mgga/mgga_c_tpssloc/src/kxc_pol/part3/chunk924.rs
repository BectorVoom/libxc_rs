//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 924/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk924<F: Float>(t2924: F, t2932: F, t2860: F, t919: F, t2904: F, t938: F, t10629: F, t315: F, t2853: F, t923: F, t2885: F, t2884: F, t307: F) -> (F, F, F, F, F, F, F) {
    let t10723 = t2924 * t2932;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    let t10756 = t315 * t10629;
    let t10760 = t2853 * t923;
    let t10765 = t919 * t2885;
    let t10770 = F::new(1.0) / t2884 / t307;
    (t10723, t10740, t10747, t10756, t10760, t10765, t10770)
}
