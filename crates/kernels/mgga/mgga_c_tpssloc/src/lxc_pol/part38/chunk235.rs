//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 235/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk235<F: Float>(t40: F, t52: F, t185: F, t607: F, t707: F, t73: F, t76: F, t145: F, t164: F, t159: F, t688: F, t690: F, t694: F, t699: F, t167: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t708 = t185 * t607;
    let t710 = 4.0 * t707 * t708;
    let t713 = piecewise3(t146, 0.0, 4.0 / 3.0 * t73 * t607);
    let t716 = piecewise3(t150, 0.0, -4.0 / 3.0 * t76 * t607);
    let t717 = t713 + t716;
    let t718 = t145 * t717;
    let t719 = t718 * t185;
    let t723 = t164 * t164;
    let t724 = 1.0 / t723;
    let t725 = t159 * t724;
    let t730 = -0.1176575e1 * t688 - 0.516475e0 * t690 - 0.2103875e0 * t694 - 0.104195e0 * t699;
    let t731 = 1.0 / t167;
    (t708, t710, t717, t718, t719, t723, t724, t725, t730, t731)
}
