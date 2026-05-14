//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1105/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1105<F: Float>(t33191: F, t1458: F, t8326: F, t3941: F, t1441: F, t1873: F, t1437: F, t8513: F, t8514: F, t1409: F, t31682: F, t8308: F, t1433: F, t31691: F, t12571: F, t8662: F) -> (F, F, F, F, F, F, F, F) {
    let t33192 = 0.135e2 * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = 27.0 * t33194;
    let t33211 = t1441 * t1873;
    let t33564 = t8513 * t8514 * t1437;
    let t33567 = t31682 * t1409;
    let t33568 = t8308 * t33567;
    let t33572 = t8513 * t31691 * t1433;
    let t33669 = t12571 * t8662;
    (t33192, t33193, t33195, t33211, t33564, t33568, t33572, t33669)
}
