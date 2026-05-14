//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 936/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk936<F: Float>(t1539: F, t23685: F, t6784: F, t23657: F, t7610: F, t23327: F, t23346: F, t23619: F, t23626: F, t23629: F, t25456: F, t25459: F, t25465: F, t25467: F, t25471: F, t6687: F, t6797: F, t7607: F) -> (F,) {
    let t25475 = t23685 * t1539;
    let t25476 = t6784 * t25475;
    let t25479 = t23657 * t7610;
    let t25482 = -0.82246703342411321825e-2 * t6687 * t25456 - 0.82246703342411321825e-2 * t6687 * t25459 - t23619 - 0.73108180748810063845e-2 * t23626 + 0.21932454224643019153e-1 * t23346 * t7607 - 0.27415567780803773942e-2 * t25465 - 0.82246703342411321825e-2 * t6687 * t25467 - 0.27415567780803773942e-2 * t23327 * t25471 + 0.27415567780803773942e-2 * t23629 + 0.27415567780803773942e-2 * t6687 * t25476 - 0.82246703342411321825e-2 * t6797 * t25479;
    (t25482,)
}
