//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1317/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1317<F: Float>(t21344: F, t2436: F, t1692: F, t1713: F, t18047: F, t19681: F, t19802: F, t19816: F, t19825: F, t21256: F, t21356: F, t2439: F, t30: F, t36547: F, t5590: F, t5591: F, t6149: F, t6153: F, t64277: F, t69887: F, t69891: F, t70213: F, t70221: F, t70227: F, t70237: F, t70241: F, t70244: F) -> (F, F) {
    let t70247 = t21344 * t2436;
    let t70251 = 3.0 / 2.0 * t2439 * t1713 * t69887 + 3.0 / 2.0 * t2439 * t1713 * t69891 + t1692 * t70213 * t30 / 2.0 - t1692 * t18047 * t21356 - t1692 * t64277 * t6153 + 3.0 / 2.0 * t2439 * t1713 * t70221 + 3.0 * t36547 * t21256 - t1692 * t5590 * t70227 / 2.0 - t1692 * t19802 * t19825 + 3.0 * t2439 * t6149 * t19681 + 2.0 * t19816 * t70237 + t19816 * t70241 - 3.0 * t19816 * t70244 - t1692 * t70247 * t5591 / 2.0;
    (t70247, t70251)
}
