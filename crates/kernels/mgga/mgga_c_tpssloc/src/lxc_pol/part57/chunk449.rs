//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 449/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk449<F: Float>(t1661: F, t3270: F, t3274: F, t4721: F, t5973: F, t5977: F, t5981: F, t1100: F, t3287: F, t1107: F, t3297: F, t5971: F, t136: F, t1113: F, t5975: F, t5979: F) -> (F, F, F, F, F, F, F) {
    let t5992 = t1661 * t1661;
    let t5993 = t3270 * t5992;
    let t5999 = t3274 - 2.0 / 9.0 * t4721 - 2.0 / 9.0 * t5973 + 2.0 / 3.0 * t5977 + t5981 / 3.0;
    let t6000 = t1100 * t5999;
    let t6006 = t3287 * t5992;
    let t6008 = t1107 * t5999;
    let t6011 = t3297 * t5971;
    let t6012 = t136 * t6011;
    let t6014 = t1113 * t5975;
    let t6015 = t136 * t6014;
    let t6017 = t1113 * t5979;
    (t5993, t6000, t6006, t6008, t6012, t6015, t6017)
}
