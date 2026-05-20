//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1608/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1608<F: Float>(t2853: F, t923: F, t2885: F, t919: F, t2884: F, t307: F, t302: F) -> (F, F, F, F) {
    let t10760 = t2853 * t923;
    let t10765 = t919 * t2885;
    let t10770 = F::new(1.0) / t2884 / t307;
    let t10771 = t302 * t10770;
    (t10760, t10765, t10770, t10771)
}
