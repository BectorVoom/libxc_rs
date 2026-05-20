//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1307/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1307<F: Float>(t5611: F, t2632: F, t39249: F, t39256: F, t39309: F, t39312: F, t75839: F, t75840: F, t75844: F, t75845: F, t75846: F, t75850: F, t75851: F) -> (F, F, F) {
    let t76001 = t5611 * t5611;
    let t76002 = t76001 * t2632;
    let t76006 = t75839 - t39249 - t75840 - t39256 - t75844 - t75845 + t75846 + t75850 + t75851 - t39309 + t39312;
    (t76001, t76002, t76006)
}
