//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1965/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1965<F: Float>(t29708: F, t3625: F, t2144: F, t6218: F, t1246: F, t27536: F, t8073: F, t1734: F, t8054: F, t3612: F, t2147: F, t6238: F) -> (F, F, F, F, F, F, F, F) {
    let t29709 = t29708 * t3625;
    let t29711 = t2144 * t6218;
    let t29712 = t29711 * t1246;
    let t29716 = t27536 * t8073;
    let t29719 = t8054 * t1734;
    let t29720 = t29719 * t1246;
    let t29723 = t29708 * t3612;
    let t29726 = t2147 * t6238;
    (t29709, t29711, t29712, t29716, t29719, t29720, t29723, t29726)
}
