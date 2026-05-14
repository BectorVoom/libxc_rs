//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1338/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1338<F: Float>(t1861: F, t19388: F, t19404: F, t19408: F, t20719: F, t20728: F, t22145: F, t22148: F, t22151: F, t5492: F, t5966: F, t6077: F, t67956: F, t67961: F, t69108: F, t69111: F, t69114: F, t69355: F) -> (F,) {
    let t72865 = t5492 * t22145 / 3.0 + 5.0 / 3.0 * t20719 * t19388 + 2.0 / 3.0 * t5492 * t22148 + 5.0 / 6.0 * t5966 * t69355 + t5492 * t22151 / 3.0 + 5.0 / 3.0 * t67956 * t6077 + 5.0 / 3.0 * t67961 * t6077 + 5.0 / 3.0 * t20728 * t19404 + 5.0 / 3.0 * t20728 * t19408 + 2.0 / 3.0 * t69108 * t1861 + 2.0 / 3.0 * t69111 * t1861 + 2.0 / 3.0 * t69114 * t1861;
    (t72865,)
}
