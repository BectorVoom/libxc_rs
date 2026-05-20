//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 184/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk184<F: Float>(t31: F, t32: F, t152: F, t164: F, t159: F, t688: F, t690: F, t694: F, t699: F, t167: F, t177: F, t172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t706 = t32 * t31;
    let t707 = t706 * t152;
    let t723 = t164 * t164;
    let t724 = F::new(1.0) / t723;
    let t725 = t159 * t724;
    let t730 = -F::new(0.1176575e1) * t688 - F::new(0.516475e0) * t690 - F::new(0.2103875e0) * t694 - F::new(0.104195e0) * t699;
    let t731 = F::new(1.0) / t167;
    let t732 = t730 * t731;
    let t738 = t177 * t177;
    let t739 = F::new(1.0) / t738;
    let t740 = t172 * t739;
    (t706, t707, t723, t724, t725, t730, t731, t732, t738, t739, t740)
}
