//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 860/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk860<F: Float>(t32344: F, t33669: F, t33677: F, t1437: F, t31860: F, t32343: F, t8513: F, t117480: F, t1433: F, t8663: F, t2098: F, t8119: F, t1851: F, t8852: F, t1858: F, t8843: F) -> (F, F, F, F, F, F, F) {
    let t124805 = t33669 * t32344;
    let t124807 = t33677 * t32344;
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    let t125050 = t2098 * t8119;
    let t125053 = t1851 * t8852;
    let t125065 = t8843 * t1858;
    (t124805, t124807, t124834, t124838, t125050, t125053, t125065)
}
