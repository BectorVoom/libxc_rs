//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1346/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1346<F: Float>(t24591: F, t85639: F, t1240: F, t3242: F, t1251: F, t2244: F, t24698: F, t491: F, t3247: F, t2127: F, t82631: F) -> (F, F, F, F, F, F) {
    let t85640 = t85639 * t24591;
    let t85642 = t1240 * t3242;
    let t85643 = t2244 * t1251;
    let t85648 = t24698 * t491;
    let t85652 = t1240 * t3247;
    let t85660 = t2127 * t82631;
    (t85640, t85642, t85643, t85648, t85652, t85660)
}
