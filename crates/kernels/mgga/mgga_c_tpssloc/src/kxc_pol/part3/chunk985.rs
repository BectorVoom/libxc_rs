//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 985/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk985<F: Float>(t13550: F, t13563: F, t10296: F, t10298: F, t10302: F, t13566: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13644: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F) -> (F, F, F, F) {
    let t14287 = 0.27785333333333333334e0 * t13550;
    let t14291 = 0.22954444444444444444e0 * t13563;
    let t14304 = -0.68863333333333333333e0 * t13566 - 0.57386111111111111112e0 * t13569 + 0.20659e1 * t13572 - 0.68863333333333333334e0 * t13575 - 0.34431666666666666667e0 * t13578 - 0.309885e1 * t13581 + 0.20659e1 * t13584 + 0.103295e1 * t13587 - 0.23154444444444444444e0 * t10296 + 0.69463333333333333333e-1 * t10302 + 0.23154444444444444444e-1 * t10298;
    let t14321 = 0.13892666666666666667e0 * t13644;
    let t14324 = 0.34431666666666666666e0 * t13602;
    let t14326 = -0.3529725e1 * t13630 - 0.17648625e1 * t13632 + 0.264729375e1 * t13635 - 0.157790625e0 * t13638 + 0.3529725e1 * t13640 - 0.11577222222222222222e0 * t13642 + t14321 - 0.104195e0 * t13647 - 0.22954444444444444444e0 * t13598 + t14324 - 0.516475e0 * t13613;
    (t14287, t14291, t14304, t14326)
}
