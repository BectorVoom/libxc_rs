//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2290/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2290<F: Float>(t13360: F, t2707: F, t1509: F, t9975: F, t242: F, t41347: F, t812: F, t40933: F, t9660: F, t10009: F, t13251: F, t13262: F, t13312: F, t2643: F, t2645: F, t2647: F, t41078: F, t41395: F, t41397: F, t41404: F, t41415: F, t41417: F, t41425: F, t41467: F, t41468: F, t4177: F, t4180: F, t4181: F, t4184: F, t46597: F, t46692: F, t9612: F, t9642: F) -> (F, F, F) {
    let t47283 = t13360 * t2707;
    let t47285 = t1509 * t9975;
    let t47307 = t812 * t41347 * t242;
    let t47308 = t40933 * t9660;
    let t47318 = F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t47283 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t13262 * t46692 * t47285 * t41078 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t41395 + t13251 * t10009 / F::cast_from(256.0_f64) + t9612 * t4177 * t4184 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t2643 * t41467 * t4181 * t41468 + t2643 * t2645 * t46597 * t2647 / F::cast_from(256.0_f64) - t9642 * t13312 / F::cast_from(512.0_f64) + t47307 * t4180 * t4181 * t47308 / F::cast_from(128.0_f64) - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t41397 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t41404 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t41415 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t41417 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t41425;
    (t47285, t47308, t47318)
}
