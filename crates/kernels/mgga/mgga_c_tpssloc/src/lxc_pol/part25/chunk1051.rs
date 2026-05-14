//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1051/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1051<F: Float>(t22527: F, t22531: F, t22534: F, t23975: F, t6492: F, t7035: F, t83832: F, t84203: F, t84205: F, t84207: F, t84209: F, t84216: F, t84220: F, t84222: F, t84224: F, t84229: F) -> (F,) {
    let t84231 = 32.0 / 3.0 * t84203 + 16.0 / 3.0 * t84205 + 32.0 / 3.0 * t84207 - 5.0 * t84209 * t6492 - 10.0 * t23975 * t22527 - 5.0 * t23975 * t22531 - 70.0 * t84216 * t83832 - 80.0 * t84220 - 8.0 / 3.0 * t84222 - 16.0 / 3.0 * t84224 - 2.0 * t22534 * t7035 + 88.0 / 9.0 * t84229;
    (t84231,)
}
