//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 964/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk964<F: Float>(t12353: F, t12356: F, t12358: F, t12361: F, t12366: F, t12371: F, t12375: F, t12379: F, t12386: F, t12388: F, t1341: F, t1363: F, t3733: F, t3778: F, t3858: F, t5246: F) -> (F,) {
    let t12390 = -5.0 / 128.0 * t1363 * t12353 - 35.0 / 384.0 * t12356 + 7.0 / 384.0 * t12358 - t1363 * t12361 / 768.0 - 119.0 / 4608.0 * t12366 - t5246 * t12371 / 128.0 + 3.0 / 16.0 * t3733 * t12375 - t1341 * t12379 / 3072.0 - t3778 * t3858 / 1024.0 - 7.0 / 768.0 * t12386 + 7.0 / 1536.0 * t12388;
    (t12390,)
}
