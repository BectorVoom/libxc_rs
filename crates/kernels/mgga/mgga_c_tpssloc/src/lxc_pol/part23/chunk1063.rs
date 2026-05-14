//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1063/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1063<F: Float>(t40590: F, t68: F, t3700: F, t195: F, t632: F, t197: F, t636: F, t39264: F, t761: F, t39259: F, t39358: F, t756: F, t187: F, t268: F, t39322: F, t39347: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0 / t40610;
    let t40632 = 1.0 / t195 / t632;
    let t40647 = 1.0 / t197 / t636;
    let t40679 = 0.61524113149298439947e4 * t761 * t39264;
    let t40685 = 0.69263436422725855036e2 * t761 * t39259;
    let t40708 = 0.18989649058080861537e-2 * t756 * t39358;
    let t40712 = t187 * t268;
    let t40714 = 0.1301229756036208781e0 * t40712 * t39322;
    let t40716 = 0.19263893255070628431e1 * t40712 * t39347;
    (t40591, t40611, t40632, t40647, t40679, t40685, t40708, t40714, t40716)
}
