//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1059/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1059<F: Float>(t236: F, t40041: F, t1336: F, t240: F, t3792: F, t10021: F, t1361: F, t22843: F, t241: F, t67: F, t1339: F, t2690: F, t3788: F, t6924: F, t246: F, t39037: F, t522: F) -> (F, F, F, F, F, F, F, F) {
    let t40042 = t40041 * t236;
    let t40044 = t1336 * t40042 * t240;
    let t40046 = t3792 * t3792;
    let t40059 = t1336 * t1361 * t10021;
    let t40070 = t241 * t22843 * t67;
    let t40123 = t1336 * t1339 * t10021;
    let t40159 = t1336 * t3788 * t2690;
    let t40167 = t6924 * t67;
    let t40168 = t40167 * t246;
    let t40224 = 840.0 * t39037 * t522;
    (t40044, t40046, t40059, t40070, t40123, t40159, t40168, t40224)
}
