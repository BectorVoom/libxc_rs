//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2652/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2652<F: Float>(t16288: F, t3853: F, t12384: F, t5234: F, t3795: F, t40281: F, t5293: F, t12156: F, t12397: F, t12429: F, t1363: F, t16257: F, t16271: F, t16275: F, t16278: F, t16401: F, t1799: F, t1827: F, t3858: F, t39973: F, t39975: F, t39983: F, t39989: F, t40070: F, t40119: F, t5289: F, t820: F) -> F {
    let t54034 = t16288 * t3853;
    let t54042 = t5234 * t12384;
    let t54043 = t54042 * t3795;
    let t54047 = t40281 * t5293;
    let t54048 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t54047;
    let t54058 = F::cast_from(35.0_f64) / F::cast_from(128.0_f64) * t1363 * t40070 * t820 * t1799 * t12156 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54034 - t16278 * t3858 / F::cast_from(1024.0_f64) - t40119 * t1827 / F::cast_from(3072.0_f64) - t12397 * t5289 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t54043 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t39973 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t39983 - t54048 - t39975 * t5293 / F::cast_from(1024.0_f64) - t12429 * t16271 / F::cast_from(512.0_f64) - t12429 * t16275 / F::cast_from(1024.0_f64) + t16401 * t16257 / F::cast_from(256.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t39989;
    t54058
}
