//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2777/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2777<F: Float>(t5614: F, t9674: F, t16859: F, t2639: F, t13360: F, t4257: F, t58181: F, t816: F, t13222: F, t13228: F, t13254: F, t13351: F, t13365: F, t16928: F, t16935: F, t2643: F, t4178: F, t46565: F, t46693: F, t46930: F, t46936: F, t46951: F, t46953: F, t46960: F, t46962: F, t46974: F, t46980: F, t46998: F, t5591: F, t831: F) -> F {
    let t58759 = t9674 * t5614;
    let t58761 = t2639 * t16859;
    let t58763 = t13360 * t4257;
    let t58765 = t58181 * t816;
    let t58789 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t46930 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t46936 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t46951 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t46953 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t58759 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t58761 - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t58763 - t58765 * t831 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t13365 * t4257 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t46960 - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t46962 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t46974 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t46980 - t4178 * t13222 * t13228 * t46565 / F::cast_from(96.0_f64) - t13254 * t16928 / F::cast_from(96.0_f64) - t4178 * t13222 * t16935 * t13351 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t46998 + t2643 * t13222 * t46693 * t5591 / F::cast_from(384.0_f64);
    t58789
}
