//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2769/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2769<F: Float>(t41115: F, t5593: F, t13258: F, t16932: F, t16937: F, t10007: F, t13080: F, t13176: F, t13244: F, t13248: F, t13251: F, t13254: F, t13262: F, t13322: F, t16836: F, t16839: F, t16841: F, t16845: F, t16907: F, t16914: F, t2643: F, t2645: F, t40951: F, t41123: F, t4177: F, t4178: F, t4180: F, t4181: F, t4184: F, t46546: F, t46737: F, t58289: F, t9632: F, t9642: F) -> F {
    let t58421 = t41115 * t5593;
    let t58425 = t13258 * t16932;
    let t58427 = t13258 * t16937;
    let t58439 = F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t46546 + t9642 * t16907 / F::cast_from(384.0_f64) + t2643 * t2645 * t16839 * t10007 / F::cast_from(768.0_f64) + t9642 * t16914 / F::cast_from(192.0_f64) + t13176 * t4177 * t4184 / F::cast_from(384.0_f64) + t16836 * t13244 / F::cast_from(384.0_f64) + t16836 * t13248 / F::cast_from(768.0_f64) - t46737 * t16841 / F::cast_from(256.0_f64) - t13262 * t4180 * t16839 * t40951 / F::cast_from(512.0_f64) + t13254 * t16845 / F::cast_from(256.0_f64) + t4178 * t4180 * t16839 * t9632 / F::cast_from(512.0_f64) + F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t58421 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t13251 * t13080 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t58425 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t58427 - t4178 * t2645 * t16839 * t41123 / F::cast_from(384.0_f64) + t4178 * t4180 * t4181 * t58289 / F::cast_from(768.0_f64) + t13251 * t13322 / F::cast_from(192.0_f64);
    t58439
}
