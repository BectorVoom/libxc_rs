//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1280/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1280<F: Float>(t1163: F, t118: F, t1273: F, t1339: F, t1663: F, t1760: F, t1834: F, t18898: F, t20288: F, t20294: F, t20396: F, t20407: F, t20640: F, t20642: F, t3502: F, t3538: F, t3542: F, t4541: F, t485: F, t5706: F, t5801: F, t5905: F, t626: F, t6309: F, t6409: F, t6437: F) -> F {
    let t20646 = -t1163 * t6309 - t118 * t20640 + t1273 * t6409 - F::new(2.0) * t1339 * t18898 - F::new(2.0) * t1339 * t20294 + t1663 * t5905 + F::new(3.0) * t1760 * t20407 - t1760 * t20642 + t1834 * t4541 - t20288 * t485 - F::new(2.0) * t20396 * t626 - F::new(2.0) * t3502 * t5801 - F::new(2.0) * t3538 * t5801 - F::new(2.0) * t3542 * t5801 + t5706 * t6437;
    t20646
}
