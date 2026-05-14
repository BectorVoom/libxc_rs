//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1328/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1328<F: Float>(t70835: F, t70885: F, t70920: F, t70963: F, t19572: F, t6243: F, t18547: F, t51635: F, t7029: F, t19441: F, t3493: F, t21011: F, t22964: F, t13856: F, t19620: F, t7310: F) -> (F, F, F, F, F, F) {
    let t70965 = t70835 + t70885 + t70920 + t70963;
    let t70986 = 2.0 * t6243 * t19572;
    let t70989 = 3.0 * t18547 * t7029 * t51635;
    let t70991 = 4.0 * t3493 * t19441;
    let t70994 = 6.0 * t18547 * t22964 * t21011;
    let t70999 = 6.0 * t19620 * t7310 * t13856;
    (t70965, t70986, t70989, t70991, t70994, t70999)
}
