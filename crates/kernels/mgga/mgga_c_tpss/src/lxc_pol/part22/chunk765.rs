//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 765/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk765<F: Float>(t4523: F, t4540: F, t1163: F, t1168: F, t118: F, t1273: F, t1322: F, t1339: F, t1600: F, t1604: F, t1663: F, t2056: F, t3491: F, t3493: F, t3499: F, t3502: F, t3538: F, t3542: F, t4341: F, t4352: F, t485: F, t488: F, t544: F, t624: F, t626: F, t646: F) -> (F, F) {
    let t4541 = t4523 + t4540;
    let t4543 = -t1163 * t1322 + t1168 * t1663 - t118 * t4341 + t1273 * t1604 - 2.0 * t1339 * t2056 - 2.0 * t1339 * t3499 - t1600 * t624 - t3491 * t485 - 2.0 * t3493 * t646 - 2.0 * t3502 * t626 - 2.0 * t3538 * t626 - 2.0 * t3542 * t626 + t4352 * t544 + t4541 * t488;
    (t4541, t4543)
}
