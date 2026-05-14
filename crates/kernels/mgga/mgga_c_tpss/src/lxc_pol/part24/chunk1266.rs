//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1266/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1266<F: Float>(t21024: F, t5706: F, t1206: F, t21011: F, t19620: F, t7029: F, t21236: F, t5532: F, t19305: F, t6106: F, t13473: F, t13547: F, t16037: F, t1684: F, t19318: F, t3493: F, t5314: F, t5512: F, t5514: F, t68801: F, t68808: F, t68810: F, t68814: F, t68817: F, t68822: F, t68826: F, t68830: F, t68833: F, t68835: F) -> (F,) {
    let t68837 = 2.0 * t5706 * t21024;
    let t68838 = t21011 * t1206;
    let t68841 = 12.0 * t19620 * t7029 * t68838;
    let t68843 = 2.0 * t21236 * t5532;
    let t68845 = 4.0 * t19305 * t6106;
    let t68846 = -4.0 * t13473 * t5514 - 2.0 * t13547 * t5514 - t16037 * t1684 - 4.0 * t19318 * t3493 - t5314 * t5512 + t68801 - t68808 - t68810 + t68814 + t68817 - t68822 + t68826 - t68830 + t68833 + t68835 + t68837 - t68841 - t68843 - t68845;
    (t68846,)
}
