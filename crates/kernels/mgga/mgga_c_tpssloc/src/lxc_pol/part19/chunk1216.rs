//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1216/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1216<F: Float>(t10984: F, t2960: F, t10213: F, t41687: F, t10857: F, t376: F, t1004: F, t10956: F, t10863: F, t3053: F, t10516: F, t3113: F, t1012: F, t1015: F, t1017: F, t10444: F) -> (F, F, F, F, F, F, F) {
    let t42622 = t2960 * t10984;
    let t42624 = t10213 * t41687;
    let t42639 = t376 * t10857;
    let t42648 = t1004 * t10956;
    let t42651 = t10863 * t3053;
    let t42653 = t3113 * t10516;
    let t42658 = t1012 * t1015 * t10444 * t1017;
    (t42622, t42624, t42639, t42648, t42651, t42653, t42658)
}
