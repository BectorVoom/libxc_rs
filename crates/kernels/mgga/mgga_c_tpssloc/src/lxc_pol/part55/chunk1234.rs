//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1234/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1234<F: Float>(t1369: F, t32717: F, t1831: F, t31165: F, t5314: F, t8466: F, t22804: F, t32711: F, t113966: F, t113982: F, t113987: F, t114000: F, t120342: F, t120344: F, t120348: F, t120350: F, t120357: F, t120363: F, t120366: F, t120369: F, t120372: F, t120375: F) -> F {
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    let t120383 = t22804 * t32711;
    let t120386 = -t120342 / F::new(1536.0) - t120344 / F::new(1536.0) - t120348 / F::new(1536.0) + F::new(7.0) / F::new(2304.0) * t120350 + F::new(5.0) / F::new(384.0) * t120357 + F::cast_from(0.56521858531796547196e-2_f64) * t113966 + F::cast_from(0.13457585364713463618e-3_f64) * t120363 - t113982 + F::cast_from(0.48447307312968469025e-2_f64) * t120366 + F::cast_from(0.48447307312968469025e-2_f64) * t120369 - F::cast_from(0.80745512188280781708e-3_f64) * t120372 + F::new(7.0) / F::new(576.0) * t113987 + F::new(7.0) / F::new(576.0) * t120375 - t120377 / F::new(384.0) - t120379 / F::new(384.0) - t120381 / F::new(384.0) + F::cast_from(0.33913115119077928318e-1_f64) * t120383 + F::cast_from(0.33913115119077928318e-1_f64) * t114000;
    t120386
}
