//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1043/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1043<F: Float>(t13546: F, t908: F, t136: F, t4389: F, t699: F, t4386: F, t10277: F, t1409: F, t2244: F, t2826: F, t4337: F, t4339: F, t690: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13547 = t908 * t13546;
    let t13548 = t136 * t13547;
    let t13550 = t699 * t4389;
    let t13551 = F::cast_from(0.21908444444444444444e0_f64) * t13550;
    let t13552 = t699 * t4386;
    let t13554 = t10277 * t1409;
    let t13555 = t13554 * t2244;
    let t13556 = t2826 * t13555;
    let t13557 = t136 * t13556;
    let t13559 = t4337 * t2244;
    let t13560 = t908 * t13559;
    let t13561 = t136 * t13560;
    let t13563 = t690 * t4339;
    (t13548, t13550, t13551, t13552, t13555, t13557, t13559, t13561, t13563)
}
