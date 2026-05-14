//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1192/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1192<F: Float>(t5611: F, t2632: F, t39249: F, t39256: F, t39309: F, t39312: F, t75839: F, t75840: F, t75844: F, t75845: F, t75846: F, t75850: F, t75851: F, t39316: F, t39320: F, t39373: F, t39397: F, t39400: F, t40679: F, t40685: F, t40708: F, t75854: F, t75855: F, t75856: F) -> (F, F, F, F) {
    let t76001 = t5611 * t5611;
    let t76002 = t76001 * t2632;
    let t76006 = t75839 - t39249 - t75840 - t39256 - t75844 - t75845 + t75846 + t75850 + t75851 - t39309 + t39312;
    let t76007 = t39316 + t39320 - t40679 - t40685 + t75854 - t75855 + t75856 + t39373 - t39397 - t39400 + t40708;
    (t76001, t76002, t76006, t76007)
}
