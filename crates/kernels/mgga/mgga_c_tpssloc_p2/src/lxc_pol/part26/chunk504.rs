//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 504/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk504<F: Float>(t2663: F, t756: F, t2373: F, t2377: F, t2408: F, t2417: F, t2423: F, t2426: F, t2429: F, t2432: F, t2450: F, t2486: F, t2518: F, t2520: F, t2530: F, t2533: F, t2537: F, t2539: F, t2654: F, t2657: F, t2661: F) -> (F, F) {
    let t2665 = F::cast_from(0.24415263074675393405e-3_f64) * t756 * t2663;
    let t2666 = -t2654 + t2373 + t2377 - t2486 + t2450 + t2518 + t2408 + t2417 + t2520 + t2539 - t2530 - t2533 - t2537 + t2657 + t2661 - t2426 + t2665 + t2429 + t2432 - t2423;
    (t2665, t2666)
}
