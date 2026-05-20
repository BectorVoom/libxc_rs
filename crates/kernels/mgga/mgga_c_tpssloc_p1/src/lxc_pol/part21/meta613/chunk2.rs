//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2385/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2385<F: Float>(t2475: F, t2461: F, t2478: F, t159: F, t172: F, t2454: F, t268: F, t39249: F, t39256: F, t39300: F, t39309: F, t39312: F, t39316: F, t39320: F, t39377: F, t39378: F, t39381: F, t39535: F, t676: F, t724: F, t732: F, t739: F, t740: F, t746: F, t747: F, t781: F, t9493: F, t9720: F, t9738: F, t9740: F, t9752: F, t9762: F, t9763: F, t9781: F, t9828: F) -> (F, F) {
    let t39661 = t2475 * t2475;
    let t39664 = t2461 * t2461;
    let t39665 = t2478 * t2478;
    let t39706 = F::cast_from(0.19964560303604640732e6_f64) * t159 / t39661 * t39664 / t39665 - F::cast_from(0.14035736694323150897e2_f64) * t9762 * t39378 * t746 + t39249 + F::cast_from(0.91082604192152556044e5_f64) * t172 * t39377 * t39378 * t39381 - F::cast_from(0.12304822629859687989e5_f64) * t172 * t39535 * t39378 * t9493 + F::cast_from(0.5848223622634646207e0_f64) * t740 * t39300 * t746 + t39256 + t39309 - t39312 - t39316 - t39320 - F::new(0.41096e0) * t268 * t9828 * t9781 - F::cast_from(0.21309037037037037036e0_f64) * t268 * t781 * t724 * t732 + F::cast_from(0.13218100589565368422e2_f64) * t268 * t676 * t9738 * t9740 - F::cast_from(0.68493333333333333332e-1_f64) * t268 * t2454 * t9752 + F::cast_from(0.38527786510141256862e1_f64) * t268 * t676 * t9720 * t9763 - F::cast_from(0.67471172535210825684e-1_f64) * t268 * t781 * t739 * t747;
    (t39664, t39706)
}
