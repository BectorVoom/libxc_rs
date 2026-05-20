//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1308/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1308<F: Float>(t39316: F, t39320: F, t39373: F, t39397: F, t39400: F, t40679: F, t40685: F, t40708: F, t75854: F, t75855: F, t75856: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40714: F, t40716: F, t40721: F, t75864: F, t75865: F) -> (F, F) {
    let t76007 = t39316 + t39320 - t40679 - t40685 + t75854 - t75855 + t75856 + t39373 - t39397 - t39400 + t40708;
    let t76009 = t39408 + t39411 - t40714 + t40716 + t75864 - t75865 + t39463 - t39468 - t40721 - t39472 - t39476;
    (t76007, t76009)
}
