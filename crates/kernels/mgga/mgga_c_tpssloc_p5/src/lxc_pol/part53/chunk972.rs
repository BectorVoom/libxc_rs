//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 972/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk972<F: Float>(t1433: F, t641: F, t8513: F, t4017: F, t79: F, t4021: F, t8307: F, t26114: F, t8327: F, t19456: F, t8326: F, t26117: F) -> (F, F, F, F, F, F, F) {
    let t119971 = t8513 * t641 * t1433;
    let t119975 = t8513 * t79 * t4017;
    let t119990 = t8513 * t8307 * t4021;
    let t120067 = F::new(2.0) * t26114 * t8327;
    let t120120 = t19456 * t8326;
    let t120121 = F::new(2.0) * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = F::new(2.0) * t120122;
    let t120124 = t26117 * t8326;
    (t119971, t119975, t119990, t120067, t120121, t120123, t120124)
}
