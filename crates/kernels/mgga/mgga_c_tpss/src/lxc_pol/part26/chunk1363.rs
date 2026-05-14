//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1363/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1363<F: Float>(t22087: F, t3154: F, t1151: F, t1153: F, t1589: F, t16015: F, t19168: F, t19172: F, t198: F, t20924: F, t20929: F, t330: F, t4023: F, t4325: F, t5297: F, t5301: F, t6044: F, t63441: F, t63448: F, t68597: F, t68601: F, t73206: F, t73253: F, t73474: F, t73540: F) -> (F,) {
    let t73546 = t22087 * t3154;
    let t73577 = t198 * t330 * (t73206 + t73253 + t73474 + t73540) * t1153 - t4023 * t73546 * t1151 - 2.0 * t4023 * t68597 * t1589 + 4.0 * t4023 * t68601 * t20929 - 2.0 * t4023 * t20924 * t4325 + 2.0 * t4023 * t63441 * t5301 - 6.0 * t4023 * t63448 * t5301 * t1151 + 4.0 * t4023 * t19172 * t1589 * t4325 - t4023 * t19168 * t5297 + 2.0 * t4023 * t19172 * t5297 * t1151 - t4023 * t6044 * t16015;
    (t73577,)
}
