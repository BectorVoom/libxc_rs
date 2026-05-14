//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1079/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1079<F: Float>(t105309: F, t105311: F, t105313: F, t105315: F, t105317: F, t105319: F, t105325: F, t105329: F, t105333: F, t105335: F, t105337: F, t105339: F, t105341: F, t105345: F, t105348: F, t84896: F, t84897: F, t98709: F, t98711: F, t98725: F) -> (F,) {
    let t108268 = -t105309 / 256.0 + t105311 / 128.0 - t105313 / 64.0 - t105315 / 192.0 - t105317 / 64.0 + 5.0 / 64.0 * t105319 - 7.0 / 8.0 * t98709 - 0.35608770875031824732e0 * t98711 - t84896 - t84897 - 0.12111826828242117256e-2 * t105325 + 0.72670960969452703536e-2 * t105329 + 0.24223653656484234512e-2 * t105333 - t105335 / 768.0 - t105337 / 256.0 - t105339 / 256.0 + 5.0 / 64.0 * t105341 + 0.84782787797694820791e-2 * t98725 + 3.0 / 8.0 * t105345 - 0.24223653656484234512e-2 * t105348;
    (t108268,)
}
