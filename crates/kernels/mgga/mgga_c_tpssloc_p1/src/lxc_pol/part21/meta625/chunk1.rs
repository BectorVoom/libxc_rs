//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2406/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2406<F: Float>(t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t2652: F, t9874: F, t2244: F, t2517: F, t2658: F, t39488: F) -> (F, F, F, F, F, F) {
    let t40712 = t187 * t268;
    let t40714 = F::cast_from(0.1301229756036208781e0_f64) * t40712 * t39322;
    let t40716 = F::cast_from(0.19263893255070628431e1_f64) * t40712 * t39347;
    let t40721 = F::cast_from(0.21053605041484726346e2_f64) * t761 * t39336;
    let t40722 = t2652 * t9874;
    let t40729 = t2658 * t2517 * t2244;
    let t40732 = F::cast_from(0.6233709278045326953e3_f64) * t761 * t39488;
    (t40714, t40716, t40721, t40722, t40729, t40732)
}
