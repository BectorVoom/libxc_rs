//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1168/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1168<F: Float>(t29895: F, t30414: F, t29900: F, t30417: F, t30420: F, t110075: F, t30407: F, t110093: F, t110141: F, t110144: F, t110526: F, t110531: F, t110533: F, t110542: F, t110564: F, t110566: F, t110586: F, t110615: F, t1453: F, t19503: F, t19529: F, t2341: F, t29903: F, t29907: F, t29922: F, t30171: F, t5396: F, t5464: F, t5468: F, t5488: F, t8128: F, t8129: F, t8137: F, t8138: F) -> (F,) {
    let t111385 = t29895 * t30414;
    let t111390 = t29900 * t30417;
    let t111395 = t29900 * t30420;
    let t111408 = t110075 * t30407;
    let t111413 = 5.0 / 9.0 * t110526 * t2341 * t1453 * t30171 - t110531 + 10.0 / 9.0 * t110533 - t110542 - 2.0 / 3.0 * t111385 - 5.0 / 12.0 * t8128 * t29907 * t5488 + 10.0 / 27.0 * t111390 + 25.0 / 108.0 * t8137 * t110093 * t5468 + 5.0 / 9.0 * t111395 + 25.0 / 72.0 * t8137 * t29922 * t5396 + 22.0 / 9.0 * t110141 - 55.0 / 27.0 * t110144 + t110564 - t110566 - t110586 + t8128 * t8129 * t19529 / 4.0 - 5.0 / 24.0 * t8137 * t8138 * t19503 + 2.0 * t111408 + 5.0 / 4.0 * t29903 * t29907 * t5464 + t110615;
    (t111413,)
}
