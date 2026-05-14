//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1107/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1107<F: Float>(t22814: F, t80782: F, t22823: F, t281: F, t22690: F, t3787: F, t1336: F, t6943: F, t836: F, t1995: F, t1999: F, t213: F, t39041: F, t557: F, t6546: F, t1365: F, t1878: F) -> (F, F, F, F, F, F, F) {
    let t80783 = t22814 * t80782;
    let t80791 = t22823 * t281;
    let t80798 = t22690 * t3787;
    let t80820 = t1336 * t6943 * t836;
    let t80825 = t39041 * t1995 * t213 * t1999;
    let t80826 = 0.10173934535723378495e0 * t80825;
    let t80827 = t6546 * t557;
    let t80830 = t1878 * t1365;
    (t80783, t80791, t80798, t80820, t80826, t80827, t80830)
}
