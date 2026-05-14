//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 611/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk611<F: Float>(t1489: F, t2563: F, t131: F, t2570: F, t205: F, t1484: F, t213: F, t221: F, t776: F, t118: F, t794: F, t2576: F, t210: F, t214: F, t4119: F, t2562: F, t2564: F, t2569: F, t2579: F, t2590: F, t787: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4124 = t2563 * t1489;
    let t4126 = t2570 * t131;
    let t4127 = t205 * t4126;
    let t4128 = t213 * t1484;
    let t4130 = t221 * t4128 * t776;
    let t4134 = t118 * t794 * t1484;
    let t4135 = t2576 * t4134;
    let t4138 = t210 * t214 * t4119;
    let t4142 = t2562 + 0.38888888888888888888e-2 * t2564 + t2569 + 0.38888888888888888887e-2 * t4124 + 0.49999999999999999998e-2 * t4127 * t4130 + 0.8333333333333333333e-3 * t4135 - 0.16666666666666666666e-2 * t787 * t4138 + 0.83333333333333333332e-3 * t2579 - t2590;
    (t4124, t4126, t4127, t4128, t4130, t4134, t4135, t4138, t4142)
}
