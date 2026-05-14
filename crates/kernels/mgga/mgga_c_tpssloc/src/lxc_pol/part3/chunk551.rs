//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 551/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk551<F: Float>(t259: F, t2592: F, t2594: F, t2597: F, t2711: F, t2713: F, t2720: F, t2743: F, t855: F, t866: F, t868: F, t261: F, t193: F, t202: F, t2486: F, t2522: F, t2523: F, t2530: F, t2533: F, t2537: F, t2539: F, t2553: F, t2654: F, t2657: F, t2661: F, t2665: F, t766: F, t776: F, t870: F) -> (F, F, F, F, F) {
    let t2745 = t259 * t2592 + 2.0 * t259 * t2594 + t259 * t2711 - 2.0 * t2597 * t866 - 2.0 * t2713 * t866 + 2.0 * t2720 * t855 - t2743 * t855;
    let t2749 = t868 * t868;
    let t2751 = t261 * t261;
    let t2752 = 1.0 / t2751;
    let t2755 = t193 * t202 * t2745 * t870 - t193 * t202 * t2749 * t2752 + 3.0 * t193 * t2553 * t766 + 6.0 * t2522 * t2523 * t776 - t2486 - t2530 - t2533 - t2537 + t2539 - t2654 + t2657 + t2661 + t2665;
    (t2745, t2749, t2751, t2752, t2755)
}
