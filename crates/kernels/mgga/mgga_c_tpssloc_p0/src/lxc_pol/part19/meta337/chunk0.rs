//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1202/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1202<F: Float>(t2374: F, t39516: F, t9879: F, t9885: F, t39325: F, t39497: F, t39500: F, t39506: F, t9882: F, t9888: F, t2430: F, t9912: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40793 = F::cast_from(0.1301229756036208781e0_f64) * t2374 * t39516;
    let t40794 = t9879 * t9885;
    let t40795 = F::cast_from(0.65061487801810439052e-1_f64) * t40794;
    let t40797 = F::cast_from(0.38025319932552508021e2_f64) * t2374 * t39325;
    let t40799 = F::cast_from(0.67471172535210825684e-1_f64) * t2374 * t39497;
    let t40801 = F::cast_from(0.86748650402413918736e-1_f64) * t2374 * t39500;
    let t40803 = F::cast_from(0.38527786510141256862e1_f64) * t2374 * t39506;
    let t40804 = t9879 * t9882;
    let t40805 = F::cast_from(0.1301229756036208781e0_f64) * t40804;
    let t40806 = t9879 * t9888;
    let t40807 = F::cast_from(0.19263893255070628431e1_f64) * t40806;
    let t40808 = t9912 * t2430;
    (t40793, t40795, t40797, t40799, t40801, t40803, t40805, t40807, t40808)
}
