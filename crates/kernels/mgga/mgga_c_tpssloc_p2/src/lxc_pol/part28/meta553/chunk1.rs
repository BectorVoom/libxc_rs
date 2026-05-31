//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1824/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1824<F: Float>(t1860: F, t1864: F, t67: F, t835: F, t22534: F, t7032: F, t23993: F, t6486: F, t24165: F, t532: F, t80743: F, t81281: F) -> (F, F, F, F, F, F) {
    let t84280 = F::cast_from(1232.0_f64) / F::cast_from(81.0_f64) * t1860 * t835 * t67 * t1864;
    let t84283 = t22534 * t7032;
    let t84285 = t6486 * t23993;
    let t84347 = t532 * t24165;
    let t84400 = F::cast_from(0.3244175520728446583e0_f64) * t80743;
    let t84423 = F::cast_from(0.19739208802178717238e0_f64) * t81281;
    (t84280, t84283, t84285, t84347, t84400, t84423)
}
