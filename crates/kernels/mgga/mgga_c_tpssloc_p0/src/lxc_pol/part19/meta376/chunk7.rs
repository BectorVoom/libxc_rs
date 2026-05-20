//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1408/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408<F: Float>(t43808: F, t43831: F, t1107: F, t11223: F, t699: F, t11205: F, t11208: F, t11219: F, t136: F, t43792: F, t3297: F, t43796: F) -> (F, F, F, F, F, F, F) {
    let t43832 = t43808 + t43831;
    let t43833 = t1107 * t43832;
    let t43835 = t699 * t11223;
    let t43837 = t699 * t11205;
    let t43839 = t699 * t11208;
    let t43842 = t136 * t11219 * t43792;
    let t43845 = t136 * t3297 * t43796;
    (t43832, t43833, t43835, t43837, t43839, t43842, t43845)
}
