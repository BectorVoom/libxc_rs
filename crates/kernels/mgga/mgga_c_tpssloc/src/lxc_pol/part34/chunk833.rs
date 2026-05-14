//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 833/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk833<F: Float>(t21592: F, t21593: F, t360: F, t1021: F, t248: F, t1044: F, t21134: F, t21138: F, t1020: F, t1041: F, t1622: F, t17607: F, t18042: F, t21562: F, t21566: F, t21570: F, t21574: F, t21580: F, t3070: F, t4641: F, t4644: F, t5857: F, t5861: F, t5869: F, t5900: F, t973: F) -> (F, F) {
    let t21594 = t21592 + t21593;
    let t21595 = t21594 * t360;
    let t21597 = t248 * t1021 * t21595;
    let t21603 = t248 * t1044 * t21134;
    let t21609 = t248 * t1044 * t21138;
    let t21612 = t973 * t21562 / 48.0 + t3070 * t21566 / 1536.0 + 5.0 / 4608.0 * t3070 * t21570 + t3070 * t21574 / 1536.0 - t4644 * t5900 / 768.0 - 5.0 / 2304.0 * t1041 * t21580 + t18042 / 1152.0 + t17607 * t1622 / 1536.0 + t4641 * t5869 / 1024.0 + t1020 * t21597 / 3072.0 + t4644 * t5857 / 1536.0 + t1041 * t21603 / 4608.0 + 5.0 / 4608.0 * t4644 * t5861 + t1041 * t21609 / 768.0;
    (t21594, t21612)
}
