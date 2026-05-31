//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 962/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk962<F: Float>(t22174: F, t471: F, t21762: F, t248: F, t3585: F, t21510: F, t4987: F, t4582: F, t1227: F, t15503: F, t15507: F, t15569: F, t15740: F, t18357: F, t18372: F, t18376: F, t18393: F, t18972: F, t18976: F, t22154: F, t22158: F, t22162: F, t22169: F, t3577: F, t488: F, t5002: F, t5005: F, t5019: F, t6192: F, t6203: F, t6221: F, t6227: F, t6232: F) -> F {
    let t22175 = t471 * t22174;
    let t22185 = t248 * t3585 * t21762;
    let t22196 = t4987 * t21510;
    let t22197 = t4582 * t22196;
    let t22202 = -t3577 * t22154 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t3577 * t22158 - t3577 * t22162 / F::cast_from(1536.0_f64) + t15569 * t6192 / F::cast_from(144.0_f64) - t15740 * t6192 / F::cast_from(768.0_f64) + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t22169 * t488 - F::cast_from(209.0_f64) / F::cast_from(2592.0_f64) * t22175 * t488 + t18357 / F::cast_from(768.0_f64) - t18372 / F::cast_from(1152.0_f64) + t18376 / F::cast_from(1536.0_f64) + t5002 * t6221 / F::cast_from(1024.0_f64) - t18393 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1227 * t22185 - t5019 * t6221 / F::cast_from(192.0_f64) - t15503 * t6227 / F::cast_from(96.0_f64) + t15507 * t6232 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t5005 * t6203 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1227 * t22197 + t18972 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t18976;
    t22202
}
