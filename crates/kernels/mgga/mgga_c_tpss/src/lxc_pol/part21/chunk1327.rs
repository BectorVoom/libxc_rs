//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1327/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1327<F: Float>(t10412: F, t578: F, t10416: F, t10425: F, t1680: F, t18325: F, t18328: F, t18363: F, t19352: F, t19411: F, t19414: F, t19417: F, t5503: F, t5507: F, t6073: F, t6087: F) -> (F,) {
    let t65234 = t578 * t10412;
    let t65237 = t578 * t10416;
    let t65244 = t578 * t10425;
    let t65249 = -t19352 * t5503 / 3.0 - t19352 * t5507 / 3.0 - t6073 * t18325 / 6.0 - t6073 * t18328 / 3.0 + t18363 * t6087 / 3.0 + 2.0 / 3.0 * t19411 * t5507 + t65234 * t1680 / 3.0 + 2.0 / 3.0 * t65237 * t1680 + 2.0 / 3.0 * t19414 * t5503 + 2.0 / 3.0 * t19414 * t5507 + t65244 * t1680 / 3.0 + 2.0 / 3.0 * t19417 * t5503;
    (t65249,)
}
