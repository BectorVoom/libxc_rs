//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1346/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1346<F: Float>(t1642: F, t60706: F, t12853: F, t5728: F, t18450: F, t4462: F, t12960: F, t5721: F, t60723: F, t65604: F, t65608: F, t65611: F, t65614: F, t65617: F, t65618: F, t65620: F, t65622: F) -> (F,) {
    let t65624 = t60706 * t1642;
    let t65626 = t5728 * t12853;
    let t65628 = t18450 * t4462;
    let t65629 = 7.0 / 1152.0 * t65628;
    let t65630 = t5721 * t12960;
    let t65632 = t65604 / 192.0 - t65608 / 256.0 + t65611 / 8.0 + t65614 / 16.0 - t65617 + 5.0 / 192.0 * t65618 + 5.0 / 384.0 * t65620 - t65622 / 1536.0 - t60723 - 119.0 / 6912.0 * t65624 - 5.0 / 64.0 * t65626 + t65629 - t65630 / 1536.0;
    (t65632,)
}
