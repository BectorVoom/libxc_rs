//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 597/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk597<F: Float>(t958: F, t962: F, t2004: F, t332: F, t917: F, t921: F, t215: F, t334: F, t671: F, t333: F, t2465: F, t970: F) -> (F, F, F, F, F, F) {
    let t2660 = t958 * t962;
    let t2662 = t2004 * t332;
    let t2665 = t917 * t921;
    let t2668 = t215 * t671 * t334;
    let t2670 = t333 * t2668 / F::new(432.0);
    let t2671 = t970 * t2465;
    (t2660, t2662, t2665, t2668, t2670, t2671)
}
