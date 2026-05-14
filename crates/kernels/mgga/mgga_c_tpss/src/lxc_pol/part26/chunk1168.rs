//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1168/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1168<F: Float>(t19476: F, t4419: F, t1642: F, t18450: F, t18454: F, t4425: F, t4462: F, t5721: F, t4466: F, t4473: F, t1646: F, t18464: F, t4480: F, t5728: F, t4484: F, t18435: F, t18437: F, t18451: F, t18462: F, t18465: F, t19466: F, t19471: F, t19473: F) -> (F, F, F) {
    let t19477 = t19476 * t4419;
    let t19479 = t18450 * t1642;
    let t19481 = t18454 * t4425;
    let t19483 = t5721 * t4462;
    let t19485 = t18454 * t4466;
    let t19489 = t18454 * t4473;
    let t19491 = t18464 * t1646;
    let t19493 = t5728 * t4480;
    let t19495 = t5728 * t4484;
    let t19497 = t18435 + 7.0 / 144.0 * t18437 + 7.0 / 144.0 * t19466 + t19471 / 16.0 - t19473 / 48.0 + t19477 / 768.0 + 7.0 / 2304.0 * t19479 + t19481 / 384.0 - t19483 / 1536.0 - t19485 / 1536.0 + 7.0 / 2304.0 * t18451 + t18462 + 7.0 / 576.0 * t18465 + t19489 / 384.0 + 7.0 / 576.0 * t19491 + 5.0 / 384.0 * t19493 - t19495 / 384.0;
    (t19479, t19491, t19497)
}
