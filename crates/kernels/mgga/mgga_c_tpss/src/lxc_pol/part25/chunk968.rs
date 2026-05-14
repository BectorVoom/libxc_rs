//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 968/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk968<F: Float>(t1373: F, t1375: F, t14268: F, t14274: F, t14282: F, t14285: F, t14291: F, t14294: F, t222: F, t224: F, t3650: F, t3656: F, t3658: F, t3661: F, t4748: F, t4752: F, t4755: F, t776: F, t779: F) -> (F,) {
    let t14297 = 6.0 * t1373 * t3661 + 6.0 * t1375 * t3650 - t14268 * t224 - 24.0 * t14274 * t3658 + 60.0 * t14282 * t3656 - 24.0 * t14285 * t3656 - 12.0 * t14291 * t3656 + 3.0 * t14294 * t222 + 3.0 * t4748 * t779 - 12.0 * t4752 * t776 + 3.0 * t4755 * t776;
    (t14297,)
}
