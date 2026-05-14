//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1255/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1255<F: Float>(t42340: F, t42341: F, t43288: F, t23508: F, t43292: F, t1049: F, t1058: F, t1060: F, t10857: F, t11034: F, t11037: F, t11040: F, t11049: F, t11055: F, t11059: F, t11060: F, t11081: F, t3187: F, t3200: F, t3201: F, t43483: F, t43489: F, t43504: F, t43525: F, t43553: F, t43554: F, t43558: F, t43562: F) -> (F,) {
    let t43576 = t42340 * t42341 * t43288;
    let t43577 = t23508 * t43292;
    let t43584 = 4.0 * t1049 * t1058 * t1060 * t10857 + 24.0 * t11059 * t11060 * t43483 + 36.0 * t11059 * t3187 * t43558 - 6.0 * t3200 * t3201 * t43489 - 3.0 * t3200 * t3201 * t43525 - 36.0 * t43504 * t43553 * t43554 + 24.0 * t43504 * t43576 * t43577 + 24.0 * t11034 * t11055 - 12.0 * t11037 * t11040 - 12.0 * t11037 * t11081 + 4.0 * t11049 * t43562;
    (t43584,)
}
