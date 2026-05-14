//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1220/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1220<F: Float>(t1299: F, t60: F, t18322: F, t19213: F, t3426: F, t3431: F, t581: F, t5971: F, t72: F, t1679: F, t5506: F, t6471: F, t5975: F, t6090: F, t1860: F, t19380: F) -> (F, F, F, F, F, F, F) {
    let t20760 = t1299 * t60;
    let t20767 = 20.0 / 9.0 * t20760 * t581 + 5.0 / 18.0 * t19213 * t3426 - 5.0 / 6.0 * t5971 * t3431 - t18322;
    let t20768 = t20767 * t72;
    let t20769 = t20768 * t1679;
    let t20772 = t6471 * t5506;
    let t20777 = t5975 * t6090;
    let t20780 = t1860 * t19380;
    (t20760, t20767, t20768, t20769, t20772, t20777, t20780)
}
