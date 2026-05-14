//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 933/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk933<F: Float>(t10735: F, t10745: F, t10751: F, t10755: F, t10758: F, t10761: F, t1373: F, t1375: F, t222: F, t224: F, t2353: F, t2358: F, t2361: F, t3650: F, t3656: F, t3658: F, t3661: F, t776: F, t779: F) -> (F,) {
    let t10764 = -t10735 * t224 - 24.0 * t10745 * t3658 + 60.0 * t10751 * t3656 - 24.0 * t10755 * t3656 - 12.0 * t10758 * t3656 + 3.0 * t10761 * t222 - 12.0 * t1373 * t2358 + 3.0 * t1373 * t2361 + 3.0 * t1375 * t2353 + 6.0 * t3650 * t779 + 6.0 * t3661 * t776;
    (t10764,)
}
