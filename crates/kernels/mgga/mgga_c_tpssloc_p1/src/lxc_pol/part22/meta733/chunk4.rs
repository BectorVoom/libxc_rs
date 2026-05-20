//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2408/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408<F: Float>(t48157: F, t60192: F, t60194: F, t60202: F, t68571: F, t68577: F, t68580: F, t68583: F, t68586: F, t68589: F, t68592: F, t42086: F, t59663: F, t59665: F, t59680: F, t59688: F, t59694: F, t60204: F, t68596: F, t68599: F, t68602: F, t68605: F, t68608: F) -> (F, F) {
    let t68839 = -F::cast_from(0.91285185185185185187e-1_f64) * t48157 - F::cast_from(0.29896666666666666667e0_f64) * t68571 + F::cast_from(0.98587999999999999998e0_f64) * t60192 - F::cast_from(0.65725333333333333332e0_f64) * t60194 - F::cast_from(0.32862666666666666666e0_f64) * t60202 + F::new(0.71752e1) * t68577 - F::new(0.53814e1) * t68580 + F::new(0.17938e1) * t68583 + F::new(0.17938e1) * t68586 + F::cast_from(0.59793333333333333334e0_f64) * t68589 - F::cast_from(0.19931111111111111111e0_f64) * t68592;
    let t68851 = F::cast_from(0.39862222222222222223e1_f64) * t68596 - F::cast_from(0.99655555555555555554e0_f64) * t68599 + F::new(0.35876e1) * t68602 - F::cast_from(0.99655555555555555555e0_f64) * t68605 - F::new(0.53814e1) * t68608 - F::cast_from(0.91285185185185185184e-1_f64) * t60204 - F::cast_from(0.59793333333333333334e0_f64) * t59663 + F::cast_from(0.19931111111111111111e0_f64) * t59665 + F::cast_from(0.29896666666666666667e0_f64) * t59680 + F::cast_from(0.79724444444444444444e0_f64) * t59688 - F::cast_from(0.39862222222222222223e0_f64) * t59694 + t42086;
    (t68839, t68851)
}
