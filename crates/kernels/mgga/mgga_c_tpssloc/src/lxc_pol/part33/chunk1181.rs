//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1181/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1181<F: Float>(t105309: F, t105311: F, t105313: F, t105315: F, t105317: F, t105319: F, t105325: F, t105329: F, t105333: F, t105335: F, t105337: F, t105339: F, t105341: F, t105345: F, t105348: F, t81850: F, t81853: F, t98709: F, t98711: F, t98725: F) -> (F,) {
    let t105350 = -t105309 / 512.0 + t105311 / 256.0 - t105313 / 128.0 - t105315 / 384.0 - t105317 / 128.0 + 5.0 / 128.0 * t105319 - 7.0 / 16.0 * t98709 - 0.17804385437515912366e0 * t98711 - t81850 - t81853 - 0.60559134141210586281e-3 * t105325 + 0.36335480484726351768e-2 * t105329 + 0.12111826828242117256e-2 * t105333 - t105335 / 1536.0 - t105337 / 512.0 - t105339 / 512.0 + 5.0 / 128.0 * t105341 + 0.42391393898847410397e-2 * t98725 + 3.0 / 16.0 * t105345 - 0.12111826828242117256e-2 * t105348;
    (t105350,)
}
