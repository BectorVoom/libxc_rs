//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 424/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk424<F: Float>(t1465: F, t947: F, t242: F, t1407: F, t970: F, t1461: F, t923: F, t925: F, t946: F, t964: F, t967: F) -> (F, F, F) {
    let t1466 = t947 * t1465;
    let t1467 = t242 * t1466;
    let t1470 = t970 * t1407;
    let t1471 = t242 * t1470;
    let t1474 = t923 + t925 * t1461 / F::new(288.0) + t946 * t1467 / F::new(3072.0) + t964 + t967 * t1471 / F::new(4608.0);
    (t1467, t1471, t1474)
}
