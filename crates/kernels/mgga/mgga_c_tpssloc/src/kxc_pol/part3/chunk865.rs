//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 865/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk865<F: Float>(t334: F, t371: F, t533: F, t556: F, t1184: F, t460: F, t590: F, t60: F, t3931: F, t580: F, t1395: F, t1404: F) -> (F, F, F, F, F, F) {
    let t6793 = t371 * t334;
    let t6924 = F::new(1.0) / t556 / t533;
    let t7319 = t1184 * t460;
    let t8705 = F::new(1.0) / t60 / t590;
    let t9203 = t3931 * t580;
    let t9205 = t1395 * t1404;
    (t6793, t6924, t7319, t8705, t9203, t9205)
}
