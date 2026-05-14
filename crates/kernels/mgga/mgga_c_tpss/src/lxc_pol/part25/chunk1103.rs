//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1103/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1103<F: Float>(t19476: F, t4419: F, t1642: F, t18450: F, t18454: F, t4425: F, t4462: F, t5721: F, t4466: F, t4473: F, t1646: F, t18464: F, t4480: F, t5728: F, t4484: F, t1705: F, t4487: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19477 = t19476 * t4419;
    let t19479 = t18450 * t1642;
    let t19481 = t18454 * t4425;
    let t19483 = t5721 * t4462;
    let t19485 = t18454 * t4466;
    let t19489 = t18454 * t4473;
    let t19491 = t18464 * t1646;
    let t19493 = t5728 * t4480;
    let t19495 = t5728 * t4484;
    let t19506 = t1705 * t4487;
    (t19477, t19479, t19481, t19483, t19485, t19489, t19491, t19493, t19495, t19506)
}
