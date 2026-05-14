//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1019/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1019<F: Float>(t1539: F, t3121: F, t3071: F, t3048: F, t4571: F, t10390: F, t10891: F, t10904: F, t10937: F, t10957: F, t14006: F, t14009: F, t14012: F, t14015: F, t14018: F, t14027: F, t14033: F, t14037: F, t1622: F, t3070: F, t3098: F, t4575: F, t4596: F, t4600: F, t4644: F, t973: F) -> (F,) {
    let t14040 = t1539 * t3121;
    let t14041 = t3071 * t14040;
    let t14049 = t3048 * t4571 / 648.0;
    let t14050 = -t973 * t14006 / 144.0 - t973 * t14009 / 36.0 + t973 * t14012 / 108.0 + t973 * t14015 / 216.0 + 7.0 / 648.0 * t973 * t14018 - t10904 * t4596 / 144.0 + t10891 * t4600 / 288.0 + t14027 + t10390 * t4575 / 2304.0 - t10937 * t4575 / 432.0 + t3070 * t14033 / 4608.0 + 5.0 / 13824.0 * t3070 * t14037 + t3070 * t14041 / 4608.0 - t4644 * t3098 / 2304.0 + 19.0 / 2592.0 * t10957 * t1622 - t14049;
    (t14050,)
}
