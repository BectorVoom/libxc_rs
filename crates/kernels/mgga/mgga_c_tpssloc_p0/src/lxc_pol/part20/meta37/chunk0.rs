//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 270/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk270<F: Float>(t145: F, t717: F, t185: F, t164: F, t159: F, t688: F, t690: F, t694: F, t699: F) -> (F, F, F, F, F, F) {
    let t718 = t145 * t717;
    let t719 = t718 * t185;
    let t723 = t164 * t164;
    let t724 = F::new(1.0) / t723;
    let t725 = t159 * t724;
    let t730 = -F::new(0.1176575e1) * t688 - F::new(0.516475e0) * t690 - F::new(0.2103875e0) * t694 - F::new(0.104195e0) * t699;
    (t718, t719, t723, t724, t725, t730)
}
