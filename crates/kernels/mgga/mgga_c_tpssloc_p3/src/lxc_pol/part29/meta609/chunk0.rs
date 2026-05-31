//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2048/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2048<F: Float>(t1914: F, t40772: F, t3034: F, t336: F, t221: F, t697: F, t1016: F, t1081: F, t2752: F, t1864: F, t2241: F, t608: F, t9231: F) -> (F, F, F, F, F, F, F) {
    let t82312 = t1914 * t40772;
    let t82510 = F::cast_from(1.0_f64) / t3034 / t336;
    let t82631 = t221 * t697;
    let t82985 = F::cast_from(1.0_f64) / t3034 / t1016;
    let t83555 = t2752 * t1081;
    let t83718 = t1864 * t2241;
    let t83722 = t9231 * t608;
    (t82312, t82510, t82631, t82985, t83555, t83718, t83722)
}
