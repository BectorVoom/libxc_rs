//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 384/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk384<F: Float>(t1454: F, t626: F, t1472: F, t751: F, t1409: F, t707: F, t1489: F, t2563: F, t118: F, t1484: F, t794: F, t2576: F, t1493: F, t225: F) -> (F, F, F, F, F, F) {
    let t4041 = t626 * t1454;
    let t4100 = t1472 * t751;
    let t4101 = t751 * t1409;
    let t4102 = t707 * t4101;
    let t4124 = t2563 * t1489;
    let t4134 = t118 * t794 * t1484;
    let t4135 = t2576 * t4134;
    let t4147 = t1493 * t225;
    (t4041, t4100, t4102, t4124, t4135, t4147)
}
