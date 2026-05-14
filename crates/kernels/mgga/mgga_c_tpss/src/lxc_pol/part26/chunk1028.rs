//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1028/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1028<F: Float>(t14297: F, t226: F, t773: F, t774: F, t124: F, t14029: F, t762: F, t2383: F, t4771: F, t801: F, t4775: F, t2143: F, t4712: F, t4708: F, t8167: F, t10654: F, t14252: F, t14254: F, t14258: F, t761: F, t771: F, t797: F, t8177: F, t8188: F) -> (F, F, F, F, F) {
    let t14298 = t14297 * t226;
    let t14300 = t773 * t774 * t14298;
    let t14303 = t124 * t14029;
    let t14304 = t762 * t14303;
    let t14308 = t2383 * t4771;
    let t14311 = t801 * t774 * t14029;
    let t14314 = t2383 * t4775;
    let t14316 = t2143 * t4712;
    let t14318 = t8167 * t4708;
    let t14320 = -7.0 / 2304.0 * t14252 + 7.0 / 4608.0 * t14254 + 5.0 / 768.0 * t797 * t14258 - t771 * t14300 / 3072.0 - t10654 - t761 * t14304 / 48.0 - 35.0 / 216.0 * t8177 - t8188 - 35.0 / 1152.0 * t14308 - t797 * t14311 / 768.0 + 7.0 / 1152.0 * t14314 + 7.0 / 144.0 * t14316 - 7.0 / 48.0 * t14318;
    (t14298, t14300, t14304, t14311, t14320)
}
