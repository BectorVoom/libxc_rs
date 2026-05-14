//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1261/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1261<F: Float>(t65600: F, t236: F, t339: F, t60698: F, t18464: F, t4480: F, t1642: F, t60706: F, t18450: F, t4462: F, t4473: F, t60738: F, t4484: F, t1646: F, t60749: F, t19506: F, t5570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65601 = 7.0 / 24.0 * t65600;
    let t65607 = t339 * t60698 * t236;
    let t65616 = t18464 * t4480;
    let t65617 = 35.0 / 288.0 * t65616;
    let t65624 = t60706 * t1642;
    let t65628 = t18450 * t4462;
    let t65629 = 7.0 / 1152.0 * t65628;
    let t65639 = t60738 * t4473;
    let t65640 = 7.0 / 288.0 * t65639;
    let t65643 = t18464 * t4484;
    let t65644 = 7.0 / 288.0 * t65643;
    let t65647 = t60749 * t1646;
    let t65667 = t19506 * t5570;
    (t65601, t65607, t65617, t65624, t65629, t65640, t65644, t65647, t65667)
}
