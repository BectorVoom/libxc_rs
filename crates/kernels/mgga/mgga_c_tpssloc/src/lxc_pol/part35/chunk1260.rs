//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1260/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1260<F: Float>(t26193: F, t28209: F, t6888: F, t1985: F, t20608: F, t6889: F, t80640: F, t7691: F, t97511: F, t20601: F, t214: F, t225: F, t567: F, t22685: F, t28191: F, t28232: F) -> (F, F, F, F, F, F) {
    let t107230 = t6888 * t26193 * t28209;
    let t107238 = t1985 * t6889 * t80640 * t20608;
    let t107250 = t6888 * t97511 * t7691;
    let t107260 = t1985 * t214 * t20601 * t225 * t567;
    let t107265 = t22685 * t26193 * t28191;
    let t107268 = t1985 * t26193 * t28232;
    (t107230, t107238, t107250, t107260, t107265, t107268)
}
