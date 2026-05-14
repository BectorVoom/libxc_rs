//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1180/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1180<F: Float>(t30: F, t823: F, t3683: F, t14076: F, t17930: F, t1364: F, t580: F, t3610: F, t1369: F, t17946: F, t136: F, t238: F, t1693: F, t215: F, t3622: F, t5547: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19671 = t823 * t30;
    let t19672 = t19671 * t3683;
    let t19678 = t17930 * t14076;
    let t19681 = t580 * t1364;
    let t19685 = t30 * t3610;
    let t19693 = t17946 * t1369;
    let t19695 = t238 * t136;
    let t19696 = t1693 * t19695;
    let t19697 = t215 * t3683;
    let t19698 = t19696 * t19697;
    let t19700 = t5547 * t3622;
    (t19671, t19672, t19678, t19681, t19685, t19693, t19695, t19696, t19697, t19698, t19700)
}
