//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1155/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1155<F: Float>(t1016: F, t3034: F, t835: F, t39063: F, t7245: F, t50: F, t9300: F, t11588: F, t2127: F, t221: F, t1240: F, t3242: F, t3247: F, t82631: F) -> (F, F, F, F, F, F, F, F) {
    let t82985 = 1.0 / t3034 / t1016;
    let t83803 = 1232.0 / 27.0 * t835;
    let t85501 = t39063 * t7245;
    let t85539 = t50 * t9300;
    let t85639 = t2127 * t221 * t11588;
    let t85642 = t1240 * t3242;
    let t85652 = t1240 * t3247;
    let t85660 = t2127 * t82631;
    (t82985, t83803, t85501, t85539, t85639, t85642, t85652, t85660)
}
