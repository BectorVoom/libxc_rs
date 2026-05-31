//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1227/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1227<F: Float>(t105387: F, t105390: F, t105393: F, t105396: F, t105402: F, t105404: F, t105406: F, t105412: F, t105415: F, t84921: F, t84932: F, t87387: F, t87403: F, t87405: F, t87432: F, t87445: F, t98828: F, t98830: F, t98836: F, t98838: F) -> F {
    let t108309 = -F::cast_from(0.50869672678616892475e-1_f64) * t105387 + F::cast_from(0.50869672678616892474e-1_f64) * t105390 - F::cast_from(0.40372756094140390853e-3_f64) * t105393 - F::cast_from(0.18975195364245983701e-1_f64) * t87387 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t105396 + F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t87403 - F::cast_from(0.31625325607076639502e-2_f64) * t87405 - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t98828 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t98830 + t105402 / F::cast_from(128.0_f64) - t105404 / F::cast_from(128.0_f64) - t105406 / F::cast_from(768.0_f64) - F::cast_from(0.16956557559538964158e-1_f64) * t98836 - F::cast_from(0.67826230238155856633e-1_f64) * t87432 - F::cast_from(0.10173934535723378495e0_f64) * t98838 - t84921 + F::cast_from(0.60559134141210586279e-3_f64) * t87445 - t84932 + t105412 / F::cast_from(64.0_f64) + F::cast_from(0.24223653656484234512e-2_f64) * t105415;
    t108309
}
