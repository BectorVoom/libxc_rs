//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1183/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1183<F: Float>(t31814: F, t33: F, t1497: F, t2436: F, t1317: F, t5506: F, t1679: F, t3486: F, t1290: F, t7682: F, t1981: F, t3426: F, t3432: F, t10292: F, t582: F, t6090: F, t619: F) -> (F, F, F, F, F, F, F, F, F) {
    let t64879 = t31814 * t33;
    let t64975 = t2436 * t1497;
    let t65157 = t5506 * t1317;
    let t65165 = t1679 * t3486;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    let t65189 = t10292 * t582;
    let t65208 = t6090 * t619;
    (t64879, t64975, t65157, t65165, t65169, t65172, t65175, t65189, t65208)
}
