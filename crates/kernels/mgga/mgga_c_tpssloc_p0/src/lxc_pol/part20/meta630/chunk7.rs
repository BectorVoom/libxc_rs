//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2291/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2291<F: Float>(t2631: F, t776: F, t13297: F, t9573: F, t13080: F, t9638: F, t13222: F, t13228: F, t13262: F, t13365: F, t210: F, t2379: F, t2643: F, t2647: F, t2707: F, t41427: F, t41435: F, t41437: F, t4158: F, t4172: F, t4178: F, t4180: F, t4181: F, t46426: F, t46693: F, t47285: F, t820: F, t843: F, t847: F, t9559: F, t9976: F, t9981: F, t9997: F) -> F {
    let t47320 = t2631 * t776;
    let t47333 = t9573 * t13297;
    let t47353 = t9638 * t13080;
    let t47359 = F::new(7.0) / F::new(4608.0) * t41427 + F::new(3.0) / F::new(128.0) * t13262 * t13222 * t47285 * t47320 - F::new(3.0) / F::new(256.0) * t13262 * t4180 * t4181 * t9976 - F::new(3.0) / F::new(4.0) * t9559 * t210 * t4158 * t2379 - F::new(7.0) / F::new(16.0) * t47333 + t2643 * t13222 * t46693 * t2647 / F::new(256.0) - F::new(7.0) / F::new(768.0) * t41435 + F::new(7.0) / F::new(192.0) * t41437 + F::new(7.0) / F::new(1536.0) * t4178 * t4180 * t4181 * t9981 - t843 * t847 * t820 * t46426 / F::new(768.0) - t13365 * t2707 / F::new(256.0) - t4172 * t9997 / F::new(768.0) + F::new(35.0) / F::new(384.0) * t47353 - F::new(3.0) / F::new(128.0) * t4178 * t13222 * t13228 * t47320;
    t47359
}
