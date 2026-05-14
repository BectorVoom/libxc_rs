//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 811/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk811<F: Float>(t232: F, t4180: F, t4181: F, t30714: F, t1516: F, t8343: F, t12571: F, t8301: F, t1437: F, t8307: F, t8513: F, t1409: F, t31011: F, t1433: F, t79: F, t4028: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t33103 = t12571 * t8301;
    let t33106 = t8307 * t1437;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33118 = t79 * t1433;
    let t33119 = t8513 * t33118;
    let t33151 = t4028 * t8326;
    (t32844, t32845, t32847, t33103, t33106, t33107, t33111, t33118, t33119, t33151)
}
