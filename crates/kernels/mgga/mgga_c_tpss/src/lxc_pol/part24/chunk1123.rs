//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1123/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1123<F: Float>(t15220: F, t16036: F, t1168: F, t118: F, t1273: F, t13133: F, t1339: F, t13554: F, t13565: F, t13974: F, t14001: F, t1604: F, t1663: F, t2056: F, t3493: F, t3502: F, t3538: F, t3542: F, t4352: F, t4541: F, t4641: F, t488: F, t5322: F, t544: F, t5463: F, t6103: F, t646: F) -> (F, F) {
    let t16037 = t15220 + t16036;
    let t16039 = t1168 * t5463 - t118 * t16037 + t1273 * t5322 - 4.0 * t13133 * t1339 - 4.0 * t1339 * t13554 - 2.0 * t13565 * t646 + t13974 * t488 + t14001 * t544 + 2.0 * t1604 * t4541 + 2.0 * t1663 * t4352 - 4.0 * t2056 * t4641 - 4.0 * t3493 * t3502 - 4.0 * t3493 * t3538 - 4.0 * t3493 * t3542 - 4.0 * t3538 * t6103;
    (t16037, t16039)
}
