//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1187/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1187<F: Float>(t25224: F, t28276: F, t6552: F, t1484: F, t23270: F, t25038: F, t98169: F, t20800: F, t6553: F, t6554: F, t1880: F, t28294: F, t1528: F, t17090: F, t1912: F, t28307: F, t4147: F, t67344: F, t7538: F, t82123: F, t82154: F, t98166: F, t98322: F) -> (F,) {
    let t105445 = t6552 * t25224 * t28276;
    let t105449 = t25038 * t23270 * t98169 * t1484;
    let t105453 = t6552 * t6553 * t6554 * t20800;
    let t105462 = t1880 * t25224 * t28294;
    let t105466 = -t82123 - 0.49348022005446793095e-1 * t105445 + 0.14804406601634037928e0 * t105449 - 0.16449340668482264365e-1 * t105453 + 0.24674011002723396548e-1 * t98322 + 12.0 * t4147 * t28307 - t82154 - 3.0 * t17090 * t7538 - t67344 * t1912 + 0.49348022005446793095e-1 * t105462 - 3.0 * t98166 * t1528;
    (t105466,)
}
