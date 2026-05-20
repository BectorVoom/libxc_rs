//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2657/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2657<F: Float>(t20433: F, t3866: F, t16336: F, t6431: F, t1831: F, t57021: F, t53945: F, t6396: F, t12283: F, t20450: F, t16233: F, t19871: F, t19873: F, t19876: F, t20000: F, t3805: F, t40192: F, t5246: F, t5248: F, t5250: F, t5303: F, t53928: F, t56685: F, t56687: F, t56878: F, t57081: F, t57568: F, t74090: F, t74120: F) -> F {
    let t74256 = t3866 * t20433;
    let t74258 = t16336 * t6431;
    let t74260 = t57021 * t1831;
    let t74274 = t53945 * t6396;
    let t74276 = t12283 * t20450;
    let t74286 = F::new(35.0) / F::new(192.0) * t74256 + F::new(7.0) / F::new(384.0) * t74258 + F::new(7.0) / F::new(384.0) * t74260 + t56878 * t5303 / F::new(256.0) + t16233 * t3805 * t74120 * t40192 / F::new(128.0) - F::new(3.0) / F::new(512.0) * t16233 * t5248 * t19871 * t57568 + F::new(7.0) / F::new(192.0) * t56685 - F::new(7.0) / F::new(384.0) * t56687 + t53928 - F::new(7.0) / F::new(192.0) * t74274 + F::new(35.0) / F::new(384.0) * t74276 + F::new(3.0) / F::new(512.0) * t19876 * t19873 - F::new(3.0) / F::new(512.0) * t57081 * t20000 + t5246 * t5248 * t74090 * t5250 / F::new(1536.0);
    t74286
}
