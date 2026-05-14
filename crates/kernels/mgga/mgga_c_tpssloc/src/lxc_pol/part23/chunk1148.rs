//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1148/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148<F: Float>(t10403: F, t10422: F, t21525: F, t18030: F, t4630: F, t17884: F, t4644: F, t13969: F, t21502: F, t3039: F, t1041: F, t21550: F, t135: F, t21537: F, t973: F, t21541: F) -> (F, F, F, F, F, F, F) {
    let t70535 = t10403 * t10422 * t21525;
    let t70554 = t18030 * t4630;
    let t70573 = t4644 * t17884;
    let t70597 = t3039 * t13969 * t21502;
    let t70640 = t1041 * t13969 * t21550;
    let t70655 = t973 * t135 * t21537;
    let t70660 = t973 * t135 * t21541;
    (t70535, t70554, t70573, t70597, t70640, t70655, t70660)
}
