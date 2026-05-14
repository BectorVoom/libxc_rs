//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1357/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1357<F: Float>(t15511: F, t15823: F, t15877: F, t15882: F, t15906: F, t15914: F, t19084: F, t20837: F, t4242: F, t4289: F, t6013: F, t63273: F, t63285: F, t63314: F, t68438: F, t68522: F) -> (F,) {
    let t73396 = t63273 / 10368.0 - t63285 / 6912.0 + t20837 * t4289 / 216.0 - t6013 * t15823 / 384.0 - t6013 * t15511 / 1152.0 + 5.0 / 6912.0 * t6013 * t15882 + 5.0 / 1152.0 * t6013 * t15877 - t63314 * t15906 / 1152.0 + t68522 * t4242 / 216.0 + 5.0 / 6912.0 * t19084 * t15914 - t68438;
    (t73396,)
}
