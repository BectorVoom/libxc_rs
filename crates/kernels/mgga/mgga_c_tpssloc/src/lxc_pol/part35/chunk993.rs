//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 993/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk993<F: Float>(t21510: F, t4987: F, t4582: F, t1227: F, t15503: F, t15507: F, t15569: F, t15740: F, t18357: F, t18372: F, t18376: F, t18393: F, t18972: F, t18976: F, t22154: F, t22158: F, t22162: F, t22169: F, t22175: F, t22185: F, t3577: F, t488: F, t5002: F, t5005: F, t5019: F, t6192: F, t6203: F, t6221: F, t6227: F, t6232: F) -> (F, F) {
    let t22196 = t4987 * t21510;
    let t22197 = t4582 * t22196;
    let t22202 = -t3577 * t22154 / 1536.0 + 5.0 / 4608.0 * t3577 * t22158 - t3577 * t22162 / 1536.0 + t15569 * t6192 / 144.0 - t15740 * t6192 / 768.0 + 19.0 / 576.0 * t22169 * t488 - 209.0 / 2592.0 * t22175 * t488 + t18357 / 768.0 - t18372 / 1152.0 + t18376 / 1536.0 + t5002 * t6221 / 1024.0 - t18393 / 1152.0 + 5.0 / 2304.0 * t1227 * t22185 - t5019 * t6221 / 192.0 - t15503 * t6227 / 96.0 + t15507 * t6232 / 192.0 + 5.0 / 4608.0 * t5005 * t6203 + 5.0 / 4608.0 * t1227 * t22197 + t18972 / 768.0 + 5.0 / 6912.0 * t18976;
    (t22197, t22202)
}
