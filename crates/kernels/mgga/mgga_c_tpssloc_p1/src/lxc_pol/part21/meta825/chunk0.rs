//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2900/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2900<F: Float>(t42086: F, t42087: F, t59680: F, t59684: F, t59688: F, t59692: F, t59694: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F, t60238: F, t60240: F) -> F {
    let t60513 = F::cast_from(0.19931111111111111111e0_f64) * t59680 - F::cast_from(0.29896666666666666667e0_f64) * t59684 + F::cast_from(0.26574814814814814815e0_f64) * t59688 + F::cast_from(0.11958666666666666667e1_f64) * t59692 - F::cast_from(0.13287407407407407408e0_f64) * t59694 - F::cast_from(0.54771111111111111112e-1_f64) * t60223 - F::cast_from(0.27385555555555555556e-1_f64) * t60226 - F::cast_from(0.36514074074074074075e-1_f64) * t60229 - F::cast_from(0.98587999999999999998e0_f64) * t60232 - F::cast_from(0.49293999999999999999e0_f64) * t60235 + t42086 + t42087 + F::new(0.5696775e1) * t60238 - F::new(0.3071625e0) * t60240;
    t60513
}
