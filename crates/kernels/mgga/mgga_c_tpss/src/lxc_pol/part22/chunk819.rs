//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 819/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk819<F: Float>(t5: F, t114: F, t1675: F, t1792: F, t5785: F, t5793: F, t6073: F, t6077: F, t6080: F, t6304: F, t117: F, t1600: F, t1799: F, t5812: F, t6109: F) -> (F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t115 = 1.0 < t114;
    let t6308 = piecewise3(t8, 0.0, t6073 * t1792 / 3.0 - 5.0 / 3.0 * t5785 * t6077 - 2.0 / 3.0 * t6080 * t1792 - t5793 + t1675 * t6304 / 3.0);
    let t6309 = t6308 * t117;
    let t6318 = t1600 * t1799;
    let t6323 = piecewise3(t115, 0.0, -t5812 - t6109 / 4.0);
    (t6308, t6309, t6318, t6323)
}
