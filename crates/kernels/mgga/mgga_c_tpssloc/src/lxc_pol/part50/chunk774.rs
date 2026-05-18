//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 774/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk774<F: Float>(t1060: F, t7619: F, t383: F, t7593: F, t1058: F, t1610: F, t1920: F, t1953: F, t353: F, t6687: F, t6783: F, t6797: F, t7604: F, t7607: F, t7611: F, t7615: F) -> (F, F, F) {
    let t7620 = t7619 * t1060;
    let t7622 = t383 * t7593;
    let t7624 = t6783 + F::new(0.27415567780803773942e-2) * t6687 * t7604 - F::new(0.82246703342411321825e-2) * t6687 * t7607 + F::new(0.82246703342411321825e-2) * t6797 * t7611 + F::new(0.82246703342411321825e-2) * t1920 * t7615 + t1610 * t1953 + t1058 * t7620 + t353 * t7622;
    (t7620, t7622, t7624)
}
