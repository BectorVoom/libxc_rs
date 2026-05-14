//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 922/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk922<F: Float>(t1338: F, t33822: F, t1824: F, t8788: F, t115439: F, t122503: F, t122507: F, t122510: F, t122513: F, t122518: F, t122522: F, t122526: F, t122530: F, t122533: F, t122535: F, t122540: F, t124166: F, t1332: F, t1336: F, t1352: F, t32136: F, t33841: F, t5230: F, t5250: F, t5287: F, t5334: F, t5344: F, t544: F, t553: F, t8798: F) -> (F,) {
    let t124246 = t1338 * t33822;
    let t124253 = t8788 * t1824;
    let t124273 = -t1336 * t124246 * t1352 - 0.16449340668482264365e-1 * t115439 - 0.76763589786250567037e-1 * t122503 - t1336 * t32136 * t5287 - t5344 * t124253 * t1352 - 0.16449340668482264365e-1 * t122507 + 0.6579736267392905746e-1 * t122510 - 0.3289868133696452873e-1 * t122513 + 0.6579736267392905746e-1 * t122518 + t544 * t553 * t124166 + 2.0 * t5334 * t124253 * t5250 + t5230 * t8798 + t1332 * t33841 + 0.6579736267392905746e-1 * t122522 - 0.6579736267392905746e-1 * t122526 - 0.6579736267392905746e-1 * t122530 + 0.3289868133696452873e-1 * t122533 + 0.15352717957250113407e0 * t122535 - 0.6579736267392905746e-1 * t122540;
    (t124273,)
}
