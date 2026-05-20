//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1871/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1871<F: Float>(t20856: F, t9975: F, t10080: F, t2632: F, t2728: F, t13416: F, t5585: F, t232: F, t860: F, t1510: F, t17030: F, t4295: F, t5617: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20857 = t20856 * t9975;
    let t20858 = t10080 * t20857;
    let t20861 = t20856 * t2632;
    let t20862 = t2728 * t20861;
    let t20867 = t13416 * t5585;
    let t20870 = t20856 * t232;
    let t20871 = t860 * t20870;
    let t20873 = t17030 * t1510;
    let t20876 = t4295 * t5617;
    (t20857, t20858, t20861, t20862, t20867, t20870, t20871, t20873, t20876)
}
