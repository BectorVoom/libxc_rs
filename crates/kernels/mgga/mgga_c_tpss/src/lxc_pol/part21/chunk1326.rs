//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1326/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1326<F: Float>(t10292: F, t582: F, t19345: F, t5502: F, t19403: F, t619: F, t77: F, t18351: F, t6086: F, t6090: F, t1678: F, t1985: F, t3418: F, t38: F, t41937: F, t1680: F, t18345: F, t18350: F, t18352: F, t18363: F, t19342: F, t19346: F, t23511: F, t6091: F, t62007: F, t62027: F, t62030: F, t7690: F) -> (F,) {
    let t65189 = t10292 * t582;
    let t65198 = t5502 * t19345;
    let t65202 = t77 * t19403 * t619;
    let t65205 = t6086 * t18351;
    let t65208 = t6090 * t619;
    let t65209 = t1678 * t65208;
    let t65214 = t3418 * t1985;
    let t65217 = t41937 * t38;
    let t65220 = 20.0 * t7690 * t23511 * t19342 - 10.0 / 3.0 * t65189 * t18352 - 10.0 * t62027 * t19342 - 10.0 / 3.0 * t62007 * t19346 - 10.0 * t62030 * t19342 - 10.0 / 3.0 * t18350 * t65198 - 10.0 * t18345 * t65202 - 10.0 / 3.0 * t18350 * t65205 - 10.0 / 3.0 * t18350 * t65209 + t18363 * t6091 / 3.0 + t65214 * t1680 / 3.0 - t65217 * t1680 / 6.0;
    (t65220,)
}
