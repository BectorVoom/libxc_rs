//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1173/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1173<F: Float>(t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11779: F, t11153: F, t1176: F, t11881: F, t374: F, t485: F, t486: F, t9697: F) -> (F, F, F, F, F, F, F) {
    let t45113 = t3575 * t42386;
    let t45114 = t11888 * t45113;
    let t45119 = t11914 * t45113;
    let t45124 = t820 * t11784;
    let t45128 = t820 * t11779;
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    let t45250 = F::cast_from(7.0_f64) / F::cast_from(31104.0_f64) * t485 * t374 * t9697 * t486;
    (t45114, t45119, t45124, t45128, t45192, t45197, t45250)
}
