//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1022/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1022<F: Float>(t1395: F, t3946: F, t12537: F, t576: F, t16: F, t2: F, t591: F, t21: F, t9: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39026 = t1395 * t3946;
    let t39028 = t576 * t12537;
    let t39030 = 0.7464e2 * t16;
    let t39031 = t2 * t591;
    let t39032 = 0.35904e3 * t39031;
    let t39033 = t9 * t21;
    let t39034 = 1638.0 * t39033;
    let t39035 = t587 * t598;
    let t39036 = 0.74688e4 * t39035;
    let t39037 = t14 * t2230;
    let t39038 = 0.175056e5 * t39037;
    let t39039 = t594 * t9223;
    let t39040 = 0.1822464e5 * t39039;
    let t39041 = 1.0 / t22811;
    (t39026, t39028, t39030, t39031, t39032, t39033, t39034, t39035, t39036, t39037, t39038, t39040, t39041)
}
