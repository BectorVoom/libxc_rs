//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 602/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk602<F: Float>(t1385: F, t1842: F, t3887: F, t3787: F, t68: F, t544: F, t1824: F, t562: F, t5250: F, t1825: F, t3901: F, t1380: F, t5287: F, t1338: F, t1352: F, t1834: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5325 = t1842 * t1385;
    let t5326 = t3887 * t5325;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    let t5336 = t5335 * t5250;
    let t5339 = t3901 * t1825;
    let t5341 = t1380 * t5287;
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t5345 = t5335 * t1352;
    let t5348 = t1338 * t1834;
    (t5325, t5326, t5334, t5335, t5336, t5339, t5341, t5344, t5345, t5348)
}
