//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1155/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1155<F: Float>(t30297: F, t662: F, t29900: F, t8269: F, t1449: F, t666: F, t8184: F, t30063: F, t2: F, t29903: F, t30048: F, t30049: F, t30051: F, t30175: F, t30279: F, t30281: F, t30285: F, t30288: F, t30291: F, t30294: F, t8128: F, t8137: F) -> (F, F, F, F, F, F, F, F) {
    let t30298 = t30297 * t662;
    let t30301 = t29900 * t8269;
    let t30303 = t1449 * t666;
    let t30304 = t8184 * t30303;
    let t30307 = t1449 * t662;
    let t30308 = t30063 * t30307;
    let t30311 = t8184 * t2;
    let t30314 = -t30048 - 2.0 / 3.0 * t30049 + 5.0 / 9.0 * t30051 - 2.0 / 3.0 * t30279 - 3.0 / 4.0 * t29903 * t30281 + 5.0 / 12.0 * t8128 * t30285 + t8128 * t30288 / 4.0 - 5.0 / 9.0 * t30291 - 5.0 / 12.0 * t8128 * t30294 + 25.0 / 72.0 * t8137 * t30298 + 5.0 / 9.0 * t30301 + 5.0 / 12.0 * t8128 * t30304 - 5.0 / 36.0 * t8137 * t30308 + 5.0 / 24.0 * t30175 * t30311;
    (t30298, t30301, t30303, t30304, t30307, t30308, t30311, t30314)
}
