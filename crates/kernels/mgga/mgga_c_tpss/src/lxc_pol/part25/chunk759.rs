//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 759/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk759<F: Float>(t5352: F, t5462: F, t118: F, t1322: F, t1339: F, t1600: F, t1604: F, t1663: F, t3493: F, t4631: F, t4638: F, t4641: F, t4675: F, t485: F, t488: F, t5314: F, t5322: F, t544: F, t626: F) -> (F, F) {
    let t5463 = t5352 + t5462;
    let t5465 = -t118 * t5314 - 2.0 * t1322 * t1600 - 4.0 * t1339 * t3493 + 2.0 * t1604 * t1663 - t4631 * t485 - 2.0 * t4638 * t485 - 4.0 * t4641 * t626 - 2.0 * t4675 * t626 + t488 * t5463 + t5322 * t544;
    (t5463, t5465)
}
