//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1178/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1178<F: Float>(t112892: F, t32792: F, t6547: F, t1880: F, t25329: F, t6553: F, t6571: F, t112660: F, t7488: F, t112899: F, t22986: F, t25054: F, t23204: F, t32866: F, t6562: F, t214: F, t225: F, t25160: F, t258: F) -> (F, F, F, F, F, F, F) {
    let t118851 = 0.82246703342411321825e-2 * t112892;
    let t118858 = t6547 * t32792;
    let t118859 = 0.38381794893125283518e-1 * t118858;
    let t118871 = 0.16449340668482264365e-1 * t1880 * t6553 * t6571 * t25329;
    let t118874 = 0.16449340668482264365e-1 * t1880 * t112660 * t7488;
    let t118877 = 0.3289868133696452873e-1 * t22986 * t112899 * t25054;
    let t118885 = t6562 * t23204 * t32866;
    let t118886 = 0.82246703342411321825e-2 * t118885;
    let t118892 = 0.16449340668482264365e-1 * t1880 * t214 * t25160 * t225 * t258;
    (t118851, t118859, t118871, t118874, t118877, t118886, t118892)
}
