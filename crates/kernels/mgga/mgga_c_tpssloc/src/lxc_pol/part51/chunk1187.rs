//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1187/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1187<F: Float>(t120220: F, t22633: F, t32704: F, t80650: F, t114285: F, t26338: F, t1992: F, t22635: F, t26226: F, t26331: F, t31099: F, t5308: F, t1385: F, t1799: F, t31090: F, t26215: F) -> (F, F, F, F, F, F, F, F) {
    let t120221 = 0.16449340668482264365e-1 * t120220;
    let t120226 = 0.3289868133696452873e-1 * t22633 * t80650 * t32704;
    let t120229 = 0.3289868133696452873e-1 * t22633 * t114285 * t26338;
    let t120232 = 0.9869604401089358619e-1 * t1992 * t22635 * t26226;
    let t120239 = 0.9869604401089358619e-1 * t26331 * t22635 * t31099 * t5308;
    let t120240 = t1799 * t1385;
    let t120244 = 0.6579736267392905746e-1 * t22633 * t22635 * t31090 * t120240;
    let t120247 = 0.3289868133696452873e-1 * t22633 * t114285 * t26215;
    (t120221, t120226, t120229, t120232, t120239, t120240, t120244, t120247)
}
