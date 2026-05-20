//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2318/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318<F: Float>(t1519: F, t5611: F, t21013: F, t814: F, t20937: F, t68: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40673: F, t40679: F, t46138: F, t67044: F, t67086: F, t67087: F, t67088: F, t67089: F, t67090: F, t67095: F, t67096: F) -> (F, F, F, F) {
    let t67405 = t1519 * t5611;
    let t67429 = t814 * t21013;
    let t67441 = t20937 * t68;
    let t67448 = -t39249 - t67044 + t67086 - t39256 - t67087 + t67088 - t67089 + t67090 + t46138 + t67095 - t39309 + t39312 + t39316 + t39320 - t67096 + t40673 - t40679;
    (t67405, t67429, t67441, t67448)
}
