//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2424/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424<F: Float>(t49222: F, t942: F, t951: F, t959: F, t10524: F, t1580: F, t42110: F, t42113: F, t10723: F, t13658: F, t10526: F, t10623: F, t13659: F, t13732: F, t2940: F, t4483: F, t4489: F, t49278: F, t49280: F, t49282: F, t49426: F, t49485: F, t49488: F, t49491: F) -> (F, F, F, F) {
    let t49567 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t942 * t49222 * t951;
    let t49572 = F::cast_from(0.91082604192152556044e5_f64) * t959 * t42110 * t1580 * t42113 * t10524;
    let t49575 = F::cast_from(0.51947577317044391277e2_f64) * t959 * t13658 * t10723;
    let t49585 = t49278 + t49280 + t49282 + F::cast_from(0.35089341735807877242e1_f64) * t2940 * t13732 + t49426 + F::cast_from(0.10389515463408878255e3_f64) * t4483 * t10526 + F::cast_from(0.35089341735807877242e1_f64) * t10623 * t4489 - F::cast_from(0.10389515463408878255e3_f64) * t2940 * t13659 + t49485 - t49488 - t49491;
    (t49567, t49572, t49575, t49585)
}
