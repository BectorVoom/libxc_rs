//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 947/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk947<F: Float>(t3090: F, t774: F, t3069: F, t3067: F, t3138: F, t9555: F, t294: F, t2966: F, t458: F, t8556: F, t1108: F, t8550: F) -> (F, F, F, F, F, F) {
    let t9561 = t774 * t3090;
    let t9562 = t9561 * t3069;
    let t9563 = t3067 * t9562;
    let t9573 = t3138 * t9555;
    let t9589 = t294 * t2966;
    let t9605 = t458 * t8556;
    let t9607 = t8550 * t1108 * t9605;
    (t9561, t9563, t9573, t9589, t9605, t9607)
}
