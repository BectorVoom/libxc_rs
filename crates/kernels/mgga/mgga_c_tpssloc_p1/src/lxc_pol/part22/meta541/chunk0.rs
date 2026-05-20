//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2027/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2027<F: Float>(t3691: F, t9919: F, t2367: F, t2508: F, t39378: F, t9493: F, t1294: F, t9713: F, t2405: F, t2412: F, t9479: F, t9481: F) -> (F, F, F, F, F, F) {
    let t39532 = t3691 * t9919;
    let t39535 = F::new(1.0) / t2508 / t2367;
    let t39537 = t39535 * t39378 * t9493;
    let t39539 = F::cast_from(0.12304822629859687989e5_f64) * t1294 * t39537;
    let t39540 = t3691 * t9713;
    let t39549 = F::cast_from(0.3103560775156404018e4_f64) * t9479 * t2412 * t9481 * t2405;
    (t39532, t39535, t39537, t39539, t39540, t39549)
}
