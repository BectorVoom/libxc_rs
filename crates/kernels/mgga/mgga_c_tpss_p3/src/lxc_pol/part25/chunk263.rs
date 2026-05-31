//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 263/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk263<F: Float>(t835: F, t837: F, t128: F, t834: F, t285: F, t281: F) -> (F, F, F, F, F, F) {
    let t838 = t835 * t837;
    let t839 = t128 * t838;
    let t841 = -t834 - F::cast_from(0.17808333333333333333e-1_f64) * t839;
    let t843 = F::cast_from(0.621814e-1_f64) * t841 * t285;
    let t844 = t281 * t281;
    let t845 = F::cast_from(1.0_f64) / t844;
    (t838, t839, t841, t843, t844, t845)
}
