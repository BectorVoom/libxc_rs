//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2270/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2270<F: Float>(t13248: F, t13258: F, t1484: F, t2631: F, t4233: F, t828: F, t10007: F, t13076: F, t13222: F, t13223: F, t13242: F, t13322: F, t13326: F, t13350: F, t1510: F, t232: F, t2643: F, t2645: F, t2647: F, t41063: F, t41096: F, t41108: F, t4178: F, t4181: F, t4182: F, t4240: F, t46692: F, t46693: F, t9516: F, t9616: F, t9642: F) -> (F, F) {
    let t46998 = t13258 * t13248;
    let t47012 = t1484 * t2631;
    let t47017 = t4233 * t828;
    let t47025 = t9642 * t13322 / F::cast_from(128.0_f64) + t2643 * t2645 * t13242 * t10007 / F::cast_from(256.0_f64) + t9642 * t13326 / F::cast_from(256.0_f64) + t2643 * t2645 * t4181 * t232 * t9516 / F::cast_from(768.0_f64) - t9642 * t13076 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t46998 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t13350 * t1510 * t9616 + t2643 * t13222 * t13223 * t10007 / F::cast_from(256.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t4178 * t46692 * t46693 * t4182 + t41096 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t13350 * t47012 * t2647 + t2643 * t13222 * t47017 * t2647 / F::cast_from(128.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t41108 - t41063 * t4240 / F::cast_from(1024.0_f64);
    (t47012, t47025)
}
