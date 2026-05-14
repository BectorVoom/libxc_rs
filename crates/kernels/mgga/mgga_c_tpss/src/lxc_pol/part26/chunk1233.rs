//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1233/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1233<F: Float>(t1702: F, t226: F, t4758: F, t5577: F, t21324: F, t1708: F, t21290: F, t228: F, t1396: F, t1707: F, t1710: F, t18006: F, t19727: F, t19736: F, t21291: F, t21299: F, t21308: F, t21313: F, t21317: F, t21321: F, t21326: F, t21331: F, t253: F, t4784: F, t4800: F, t5565: F, t5571: F, t6135: F, t6138: F, t6143: F, t6146: F) -> (F, F, F, F) {
    let t21335 = t1702 * t4758 * t226;
    let t21336 = t5577 * t21335;
    let t21338 = t21324 * t226;
    let t21339 = t5577 * t21338;
    let t21342 = t1708 * t228 * t21290;
    let t21344 = -2.0 * t1396 * t19727 - t1707 * t21342 - t1710 * t21299 - 4.0 * t18006 * t21313 + 4.0 * t19736 * t6138 + 2.0 * t19736 * t6143 + t21291 * t253 - 6.0 * t21308 * t5571 + 4.0 * t21317 * t5571 + 2.0 * t21321 * t5571 - 2.0 * t21326 * t5571 + 2.0 * t21331 * t5571 + t21336 * t5571 + t21339 * t5571 + 2.0 * t4784 * t5565 - t4800 * t5565 - 2.0 * t6135 * t6146;
    (t21336, t21339, t21342, t21344)
}
