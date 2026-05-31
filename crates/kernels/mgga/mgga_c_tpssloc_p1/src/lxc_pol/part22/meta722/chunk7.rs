//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2363/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2363<F: Float>(t1484: F, t16606: F, t16625: F, t16949: F, t17116: F, t20800: F, t2522: F, t2523: F, t25365: F, t39249: F, t39256: F, t39373: F, t39397: F, t39400: F, t39408: F, t39463: F, t39468: F, t39472: F, t39476: F, t39529: F, t39593: F, t40708: F, t40721: F, t40779: F, t40784: F, t4119: F, t41254: F, t4310: F, t4314: F, t46138: F, t46218: F, t46235: F, t46336: F, t57932: F, t67044: F, t67086: F, t67087: F, t67088: F, t67089: F, t67090: F, t67101: F, t67104: F, t67105: F, t67112: F, t67134: F, t67137: F, t67141: F, t67160: F, t67175: F, t67195: F, t67204: F, t67206: F, t67207: F, t67210: F, t67211: F, t67212: F, t67218: F, t67286: F, t67478: F, t67480: F, t67482: F, t67484: F, t67485: F, t67486: F, t68375: F, t68391: F, t68407: F, t68414: F, t776: F) -> F {
    let t68418 = -t39256 + t39373 - t39472 - t39468 - t39400 - t39397 + t68375 + t68391 + t68407 + t39408 - t46235 - t40779 + t46218 + t39463 - t40721 - t39593 + t40708 - t39476 + t46336 - t67044 + t67175 + t67195 - t67141 + t67104 + t67105 + t67101 + t68414 + t67134 + t67286 - t67210 + t67211 + t67086 - t67087 + t67088 + t67218 + t40784 + F::cast_from(18.0_f64) * t4314 * t4310 * t16949 + F::cast_from(3.0_f64) * t2522 * t2523 * t20800 - F::cast_from(9.0_f64) * t2522 * t17116 * t25365 - F::cast_from(9.0_f64) * t2522 * t16625 * t4119 + F::cast_from(3.0_f64) * t2522 * t67112 * t776 + F::cast_from(9.0_f64) * t2522 * t57932 * t1484 + F::cast_from(9.0_f64) * t2522 * t16606 * t4119 + t67160 + t67137 - t39249 + t67212 + t46138 + t41254 + t67485 - t67486 + t67206 + t67207 - t67089 + t67090 + t67482 + t67484 + t67478 + t67480 + t67204 - t39529;
    t68418
}
