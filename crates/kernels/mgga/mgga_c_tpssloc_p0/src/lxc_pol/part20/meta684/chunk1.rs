//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2593/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593<F: Float>(t1215: F, t2244: F, t475: F, t3242: F, t1216: F, t3493: F, t1011: F, t1212: F, t52446: F, t11539: F, t1174: F, t14736: F) -> (F, F, F, F, F) {
    let t52537 = t2244 * t1215;
    let t52538 = t52537 * t475;
    let t52548 = t475 * t3242;
    let t52554 = t1216 * t3493;
    let t52568 = t52446 * t1011 * t1212;
    let t52575 = t1174 * t11539 * t14736;
    (t52538, t52548, t52554, t52568, t52575)
}
