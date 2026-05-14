//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1237/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1237<F: Float>(t3255: F, t5918: F, t65551: F, t65561: F, t65567: F, t60696: F, t60707: F, t60709: F, t60713: F, t62375: F, t65553: F, t65555: F, t65557: F, t65559: F, t65570: F, t65592: F) -> (F, F, F, F) {
    let t67131 = t3255 * t5918;
    let t67138 = 7.0 / 576.0 * t65551;
    let t67143 = 7.0 / 144.0 * t65561;
    let t67148 = 35.0 / 108.0 * t65567;
    let t67149 = t67138 + t65553 / 96.0 - 5.0 / 96.0 * t65555 - t65557 / 48.0 - t62375 + t65559 / 384.0 - t67143 - 7.0 / 576.0 * t60696 - 119.0 / 1728.0 * t60707 + 7.0 / 1152.0 * t60709 + 7.0 / 1152.0 * t60713 - t67148;
    let t67150 = 7.0 / 36.0 * t65570;
    let t67160 = 7.0 / 288.0 * t65592;
    (t67131, t67149, t67150, t67160)
}
