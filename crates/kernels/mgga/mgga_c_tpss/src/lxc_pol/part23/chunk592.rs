//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 592/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk592<F: Float>(t2551: F, t885: F, t2453: F, t2511: F, t2455: F, t2462: F, t2467: F, t2471: F, t2489: F, t2497: F, t2505: F, t2507: F, t2513: F, t2517: F, t2520: F, t2523: F) -> (F, F, F, F) {
    let t2552 = t2551 * t885;
    let t2557 = 0.68863333333333333333e0 * t2453;
    let t2564 = 0.17365833333333333333e0 * t2511;
    let t2569 = -0.17648625e1 * t2489 + 0.3529725e1 * t2497 + t2557 + 0.34431666666666666666e0 * t2455 - 0.34431666666666666667e0 * t2462 + 0.103295e1 * t2467 - 0.516475e0 * t2471 + 0.31558125e0 * t2505 + 0.6311625e0 * t2507 + t2564 + 0.13892666666666666667e0 * t2513 - 0.34731666666666666667e-1 * t2517 + 0.20839e0 * t2520 - 0.104195e0 * t2523;
    (t2552, t2557, t2564, t2569)
}
