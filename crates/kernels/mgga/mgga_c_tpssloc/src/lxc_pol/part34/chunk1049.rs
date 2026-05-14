//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1049/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1049<F: Float>(t20416: F, t6888: F, t6889: F, t6890: F, t20465: F, t22833: F, t20475: F, t26309: F, t20460: F, t20454: F, t26233: F, t6422: F, t20565: F, t6952: F, t20556: F, t6945: F) -> (F, F, F, F, F, F, F, F) {
    let t107056 = t6888 * t6889 * t6890 * t20416;
    let t107063 = t22833 * t20465;
    let t107065 = t26309 * t20475;
    let t107067 = t22833 * t20460;
    let t107070 = t22833 * t20454;
    let t107074 = t26233 * t6422;
    let t107077 = t6952 * t20565;
    let t107084 = t6945 * t20556;
    (t107056, t107063, t107065, t107067, t107070, t107074, t107077, t107084)
}
