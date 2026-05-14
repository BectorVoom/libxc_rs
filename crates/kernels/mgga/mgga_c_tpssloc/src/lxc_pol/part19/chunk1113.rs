//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1113/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1113<F: Float>(t39358: F, t756: F, t706: F, t9448: F, t708: F, t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t2652: F, t9874: F, t2523: F, t39400: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t4314: F, t9616: F) -> (F, F, F, F, F, F, F) {
    let t40708 = 0.18989649058080861537e-2 * t756 * t39358;
    let t40709 = t706 * t9448;
    let t40711 = 16.0 * t40709 * t708;
    let t40712 = t187 * t268;
    let t40714 = 0.1301229756036208781e0 * t40712 * t39322;
    let t40716 = 0.19263893255070628431e1 * t40712 * t39347;
    let t40721 = 0.21053605041484726346e2 * t761 * t39336;
    let t40722 = t2652 * t9874;
    let t40723 = 0.22787578869697033845e-2 * t40722;
    let t40724 = 72.0 * t2523 * t4314 * t9616 - t39400 + t39408 + t39411 + t39463 - t39468 - t39472 - t39476 + t40708 + t40711 - t40714 + t40716 - t40721 - t40723;
    (t40708, t40711, t40714, t40716, t40721, t40723, t40724)
}
