//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1224/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1224<F: Float>(t84130: F, t84322: F, t84719: F, t85370: F, t12521: F, t12524: F, t12529: F, t12532: F, t1401: F, t16535: F, t20173: F, t2039: F, t2098: F, t2319: F, t2363: F, t23917: F, t24462: F, t24465: F, t24478: F, t24481: F, t3938: F, t3941: F, t45557: F, t45560: F, t55344: F, t577: F, t671: F, t7056: F, t7230: F, t7235: F, t84033: F, t84044: F, t84078: F, t9416: F) -> (F, F) {
    let t85372 = t84130 + t84322 + t84719 + t85370;
    let t85375 = F::new(81.0) * t84033 * t2319 + F::new(0.135e2) * t1401 * t84044 + F::new(81.0) * t16535 * t7056 + F::new(81.0) * t3941 * t23917 * t671 + F::new(81.0) * t3941 * t7056 * t2363 + F::new(81.0) * t55344 * t2039 + F::new(162.0) * t12524 * t24478 + F::new(81.0) * t12524 * t24481 + F::new(81.0) * t45560 * t7235 + F::new(81.0) * t20173 * t24481 + F::new(0.405e2) * t12521 * t7056 + F::new(0.405e2) * t3938 * t23917 + F::new(0.135e2) * t7230 * t9416 + F::new(0.135e2) * t45557 * t2039 + F::new(27.0) * t2098 * t12529 + F::new(27.0) * t3941 * t2039 * t9416 + F::new(0.405e2) * t84078 * t671 + F::new(81.0) * t24465 * t12532 + F::new(0.405e2) * t24462 * t2363 + F::new(0.45e1) * t85372 * t577;
    (t85372, t85375)
}
