//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 558/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk558<F: Float>(t1019: F, t1023: F, t1022: F, t404: F, t394: F, t392: F, t395: F, t2834: F, t2509: F, t275: F, t400: F, t1039: F, t673: F, t1100: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2857 = t1019 * t1023;
    let t2860 = t1022 * t404;
    let t2861 = 1.0 / t2860;
    let t2862 = t394 * t2861;
    let t2868 = 1.0 / t395 / t392;
    let t2872 = 4.0 / 9.0 * t2834;
    let t2880 = 0.39862222222222222223e0 * t2834;
    let t2885 = 1.0/f64::sqrt(t392);
    let t2891 = t275 * t2509 * t400;
    let t2892 = 0.13692777777777777778e0 * t2891;
    let t2893 = t673 * t1039;
    let t2895 = t235 * t1100;
    (t2857, t2861, t2862, t2868, t2872, t2880, t2885, t2891, t2892, t2893, t2895)
}
