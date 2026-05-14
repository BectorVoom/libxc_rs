//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 613/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk613<F: Float>(t1118: F, t6020: F, t1099: F, t3315: F, t5988: F, t3313: F, t3319: F, t4721: F, t5973: F, t5977: F, t5981: F, t1682: F, t1137: F, t3339: F, t3346: F, t4770: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F) -> (F, F, F, F, F, F, F, F) {
    let t6021 = t6020 * t1118;
    let t6023 = 1.0 * t1099 * t6021;
    let t6024 = t5988 * t3315;
    let t6026 = 0.16081979498692535067e2 * t3313 * t6024;
    let t6031 = t3319 - 0.11415555555555555555e-1 * t4721 - 0.11415555555555555555e-1 * t5973 + 0.34246666666666666666e-1 * t5977 + 0.17123333333333333333e-1 * t5981;
    let t6036 = t1682 * t1682;
    let t6037 = t6036 * t1137;
    let t6052 = -0.17648625e1 * t5993 + 0.3529725e1 * t6000 + t3339 - 0.34431666666666666666e0 * t4721 - 0.34431666666666666667e0 * t5973 + 0.103295e1 * t5977 + 0.516475e0 * t5981 + 0.31558125e0 * t6006 + 0.6311625e0 * t6008 + t3346 - 0.13892666666666666667e0 * t4770 - 0.34731666666666666667e-1 * t6012 + 0.20839e0 * t6015 + 0.104195e0 * t6018;
    (t6021, t6023, t6024, t6026, t6031, t6036, t6037, t6052)
}
