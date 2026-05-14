//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1081/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1081<F: Float>(t105387: F, t105390: F, t105393: F, t105396: F, t105402: F, t105404: F, t105406: F, t105412: F, t105415: F, t84921: F, t84932: F, t87387: F, t87403: F, t87405: F, t87432: F, t87445: F, t98828: F, t98830: F, t98836: F, t98838: F) -> (F,) {
    let t108309 = -0.50869672678616892475e-1 * t105387 + 0.50869672678616892474e-1 * t105390 - 0.40372756094140390853e-3 * t105393 - 0.18975195364245983701e-1 * t87387 - 5.0 / 32.0 * t105396 + 119.0 / 1152.0 * t87403 - 0.31625325607076639502e-2 * t87405 - 35.0 / 96.0 * t98828 + 7.0 / 48.0 * t98830 + t105402 / 128.0 - t105404 / 128.0 - t105406 / 768.0 - 0.16956557559538964158e-1 * t98836 - 0.67826230238155856633e-1 * t87432 - 0.10173934535723378495e0 * t98838 - t84921 + 0.60559134141210586279e-3 * t87445 - t84932 + t105412 / 64.0 + 0.24223653656484234512e-2 * t105415;
    (t108309,)
}
