//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3011/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3011<F: Float>(t381: F, t61719: F, t1058: F, t1060: F, t11034: F, t11046: F, t11059: F, t14488: F, t14577: F, t14630: F, t14651: F, t1629: F, t18089: F, t18100: F, t18112: F, t18139: F, t18142: F, t18151: F, t3166: F, t3180: F, t3186: F, t3188: F, t3200: F, t3201: F, t43473: F, t4678: F, t5866: F, t5932: F, t5936: F, t62945: F) -> (F, F) {
    let t62984 = t381 * t61719;
    let t62988 = t1058 * t1060 * t3166 * t5866 - F::new(2.0) * t14488 * t1629 * t3200 * t3201 + F::new(2.0) * t11046 * t14630 * t5932 + F::new(6.0) * t11059 * t14577 * t5936 + F::new(2.0) * t3186 * t3188 * t62945 - F::new(2.0) * t3200 * t3201 * t62984 + F::new(8.0) * t11034 * t18139 + F::new(8.0) * t11034 * t18142 + F::new(4.0) * t14651 * t4678 + F::new(4.0) * t18089 * t3180 + F::new(2.0) * t18100 * t3180 + F::new(12.0) * t18112 * t43473 + F::new(4.0) * t18151 * t3180;
    (t62984, t62988)
}
