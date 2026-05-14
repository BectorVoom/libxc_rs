//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1025/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1025<F: Float>(t14164: F, t14165: F, t4582: F, t10263: F, t10403: F, t1041: F, t10413: F, t10896: F, t14122: F, t14126: F, t14130: F, t14136: F, t14139: F, t14143: F, t14147: F, t14152: F, t14158: F, t14160: F, t1607: F, t2960: F, t3070: F, t3117: F, t4562: F, t4565: F, t4585: F, t973: F) -> (F,) {
    let t14166 = t14164 * t14165;
    let t14167 = t4582 * t14166;
    let t14170 = t10403 * t14122 / 2304.0 - t10413 * t14126 / 4608.0 - t3070 * t14130 / 2304.0 - t10896 / 4608.0 - t14136 + t14139 - t3117 * t4585 / 1152.0 - t1041 * t14143 / 1152.0 - t1041 * t14147 / 2304.0 + t2960 * t4562 / 27.0 + t973 * t14152 / 48.0 - 2.0 / 81.0 * t2960 * t4565 - t14158 - t14160 / 1296.0 + 11.0 / 324.0 * t10263 * t1607 + t1041 * t14167 / 768.0;
    (t14170,)
}
