//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2541/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2541<F: Float>(t11190: F, t15060: F, t3265: F, t11129: F, t11306: F, t11307: F, t11310: F, t11350: F, t11361: F, t11415: F, t11420: F, t11421: F, t15146: F, t15210: F, t15226: F, t15229: F, t1683: F, t3333: F, t3357: F, t44220: F, t4820: F, t4823: F, t4861: F, t51427: F, t51437: F, t51439: F, t51441: F, t51443: F, t51446: F) -> (F, F) {
    let t51449 = F::cast_from(0.28947563097646563121e3_f64) * t11190 * t15060 * t3265;
    let t51450 = F::cast_from(0.30762056574649219973e4_f64) * t44220 * t15226 + F::cast_from(0.10526802520742363173e2_f64) * t11361 * t15210 + F::cast_from(18.0_f64) * t11415 * t15229 + F::cast_from(0.11579025239058625248e4_f64) * t11350 * t4823 * t11306 + F::cast_from(18.0_f64) * t3357 * t4820 * t3333 + F::cast_from(6.0_f64) * t15146 * t11307 - F::cast_from(0.19298375398431042081e3_f64) * t51427 * t11421 - F::cast_from(24.0_f64) * t11420 * t1683 * t11306 + F::cast_from(0.6233709278045326953e3_f64) * t11310 * t4861 * t11129 + t51437 + t51439 - t51441 - t51443 + t51446 + t51449;
    (t51449, t51450)
}
