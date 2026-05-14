//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1238/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1238<F: Float>(t10943: F, t135: F, t973: F, t3152: F, t698: F, t10870: F, t3117: F, t1020: F, t10858: F, t248: F, t3101: F, t10961: F, t3108: F, t1025: F, t10403: F, t10426: F, t10428: F, t10480: F, t10501: F, t10517: F, t10915: F, t10949: F, t10965: F, t13980: F, t13985: F, t14213: F, t3071: F, t3098: F, t3123: F, t3130: F, t39110: F, t42639: F, t4582: F, t4594: F, t974: F, t998: F) -> (F,) {
    let t43103 = t973 * t135 * t10943;
    let t43110 = t973 * t698 * t3152;
    let t43114 = t3117 * t10870;
    let t43118 = t1020 * t248 * t3101 * t10858;
    let t43120 = t10961 * t3108;
    let t43141 = -t3117 * t10915 / 192.0 + 7.0 / 486.0 * t43103 + t973 * t974 * t998 * t39110 / 288.0 + t43110 / 108.0 + 19.0 / 288.0 * t10517 * t3123 - t43114 / 1728.0 + t43118 / 1152.0 - t43120 * t1025 / 48.0 + t10949 * t10428 / 128.0 + t3130 * t4582 * t42639 * t4594 / 384.0 + 3.0 / 256.0 * t10480 * t4582 * t10426 * t13985 - t10965 * t3098 / 384.0 - 5.0 / 576.0 * t3117 * t10501 + t10403 * t3071 * t13980 * t14213 / 192.0;
    (t43141,)
}
