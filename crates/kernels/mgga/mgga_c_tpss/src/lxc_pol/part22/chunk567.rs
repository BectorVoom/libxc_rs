//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 567/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk567<F: Float>(t2482: F, t866: F, t2481: F, t267: F, t270: F, t849: F, t2453: F, t2455: F, t2462: F, t2467: F, t2471: F, t847: F, t854: F, t235: F, t68: F, t275: F, t277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2483 = t2482 * t866;
    let t2485 = 2.0 * t2481 * t2483;
    let t2487 = 1.0 / t270 / t267;
    let t2488 = t849 * t849;
    let t2489 = t2487 * t2488;
    let t2491 = 4.0 / 9.0 * t2453;
    let t2496 = t2491 + 2.0 / 9.0 * t2455 - 2.0 / 9.0 * t2462 + 2.0 / 3.0 * t2467 - t2471 / 3.0;
    let t2497 = t847 * t2496;
    let t2499 = 0.39862222222222222223e0 * t2453;
    let t2504 = 1.0/f64::sqrt(t267);
    let t2505 = t2504 * t2488;
    let t2507 = t854 * t2496;
    let t2509 = t68 * t235;
    let t2511 = t275 * t2509 * t277;
    (t2483, t2485, t2487, t2488, t2489, t2491, t2496, t2497, t2499, t2504, t2505, t2507, t2509, t2511)
}
