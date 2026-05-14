//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 676/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk676<F: Float>(t1055: F, t5943: F, t1052: F, t1635: F, t388: F, t4557: F, t4660: F, t5849: F, t5851: F, t5915: F, t5920: F, t1637: F, t1070: F, t193: F, t3216: F, t336: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F) -> (F, F, F, F) {
    let t5944 = t1055 * t5943;
    let t5946 = 2.0 * t1052 * t5920 - t1052 * t5944 - 2.0 * t1635 * t4557 - 2.0 * t1635 * t4660 + t388 * t5849 + 2.0 * t388 * t5851 + t388 * t5915;
    let t5950 = t1637 * t1637;
    let t5954 = t1070 * t193 * t336 * t5946 - t193 * t3216 * t336 * t5950 - t5691 + t5693 - t5697 + t5729 + t5732 + t5798 + t5800 - t5802 + t5806 - t5810 - t5814;
    (t5944, t5946, t5950, t5954)
}
