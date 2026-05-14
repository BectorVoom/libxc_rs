//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 605/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk605<F: Float>(t2595: F, t904: F, t2453: F, t2511: F, t2455: F, t2462: F, t2467: F, t2471: F, t2489: F, t2497: F, t2505: F, t2507: F, t2513: F, t2517: F, t2520: F, t2523: F) -> (F, F, F, F) {
    let t2596 = t2595 * t904;
    let t2601 = 0.40256666666666666667e0 * t2453;
    let t2608 = 0.137975e0 * t2511;
    let t2613 = -0.1294625e1 * t2489 + 0.258925e1 * t2497 + t2601 + 0.20128333333333333334e0 * t2455 - 0.20128333333333333333e0 * t2462 + 0.60385e0 * t2467 - 0.301925e0 * t2471 + 0.82524375e-1 * t2505 + 0.16504875e0 * t2507 + t2608 + 0.11038e0 * t2513 - 0.27595e-1 * t2517 + 0.16557e0 * t2520 - 0.82785e-1 * t2523;
    (t2596, t2601, t2608, t2613)
}
