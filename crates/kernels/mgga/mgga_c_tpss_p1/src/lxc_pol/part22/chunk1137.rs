//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1137/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1137<F: Float>(t11810: F, t12663: F, t10445: F, t10456: F, t10461: F, t10464: F, t1163: F, t118: F, t1273: F, t1322: F, t1339: F, t1600: F, t1604: F, t2054: F, t2056: F, t2062: F, t2065: F, t3166: F, t3396: F, t3491: F, t3493: F, t3502: F, t4341: F, t4352: F, t485: F, t624: F, t626: F, t7798: F) -> (F, F) {
    let t12664 = t11810 + t12663;
    let t12669 = -t10445 * t485 - F::new(4.0) * t10456 * t1339 - F::new(4.0) * t10461 * t626 - F::new(2.0) * t10464 * t626 - F::new(2.0) * t1163 * t3491 - t118 * t12664 + F::new(2.0) * t1273 * t4352 - t1322 * t3166 - F::new(2.0) * t1339 * t7798 - t1600 * t2054 - F::new(2.0) * t1600 * t2062 + t1604 * t3396 - F::new(4.0) * t2056 * t3502 - F::new(4.0) * t2065 * t3493 - F::new(2.0) * t4341 * t624;
    (t12664, t12669)
}
