//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 873/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk873<F: Float>(t22685: F, t28191: F, t31137: F, t120317: F, t1842: F, t1992: F, t22635: F, t1985: F, t28232: F, t120544: F, t6888: F, t7691: F, t26225: F, t6439: F, t28186: F, t6889: F, t6906: F) -> (F, F, F, F, F, F) {
    let t127176 = 0.9869604401089358619e-1 * t22685 * t31137 * t28191;
    let t127180 = 0.6579736267392905746e-1 * t1992 * t22635 * t120317 * t1842;
    let t127183 = 0.3289868133696452873e-1 * t1985 * t31137 * t28232;
    let t127187 = 0.6579736267392905746e-1 * t6888 * t120544 * t7691;
    let t127197 = 0.9869604401089358619e-1 * t1992 * t22635 * t26225 * t6439;
    let t127201 = 0.16449340668482264365e-1 * t1985 * t6889 * t6906 * t28186;
    (t127176, t127180, t127183, t127187, t127197, t127201)
}
