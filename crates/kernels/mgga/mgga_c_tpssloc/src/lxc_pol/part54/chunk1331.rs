//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1331/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1331<F: Float>(t22635: F, t26331: F, t31099: F, t5308: F, t1385: F, t1799: F, t22633: F, t31090: F, t114285: F, t26215: F, t114226: F, t1992: F, t31091: F, t90566: F) -> (F, F, F, F, F, F) {
    let t120239 = F::new(0.9869604401089358619e-1) * t26331 * t22635 * t31099 * t5308;
    let t120240 = t1799 * t1385;
    let t120244 = F::new(0.6579736267392905746e-1) * t22633 * t22635 * t31090 * t120240;
    let t120247 = F::new(0.3289868133696452873e-1) * t22633 * t114285 * t26215;
    let t120253 = F::new(0.3289868133696452873e-1) * t22633 * t22635 * t114226 * t1799;
    let t120258 = F::new(0.3289868133696452873e-1) * t1992 * t90566 * t31091;
    (t120239, t120240, t120244, t120247, t120253, t120258)
}
