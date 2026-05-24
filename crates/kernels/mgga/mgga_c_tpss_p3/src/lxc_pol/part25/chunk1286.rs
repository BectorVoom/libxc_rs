//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1286/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1286<F: Float>(t1679: F, t3486: F, t1290: F, t7682: F, t1981: F, t3426: F, t3432: F, t10292: F, t582: F, t6090: F, t619: F, t61871: F) -> (F, F, F, F, F, F, F) {
    let t65165 = t1679 * t3486;
    let t65169 = t7682 * t1290;
    let t65172 = t1981 * t3426;
    let t65175 = t1981 * t3432;
    let t65189 = t10292 * t582;
    let t65208 = t6090 * t619;
    let t65437 = F::new(22.0) / F::new(9.0) * t61871;
    (t65165, t65169, t65172, t65175, t65189, t65208, t65437)
}
