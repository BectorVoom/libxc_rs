//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1091/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1091<F: Float>(t22832: F, t3777: F, t3809: F, t1336: F, t6943: F, t836: F, t1995: F, t1999: F, t213: F, t39041: F, t557: F, t6546: F) -> (F, F, F, F) {
    let t80816 = t3777 * t22832;
    let t80817 = t80816 * t3809;
    let t80820 = t1336 * t6943 * t836;
    let t80821 = t80820 * t3809;
    let t80825 = t39041 * t1995 * t213 * t1999;
    let t80827 = t6546 * t557;
    (t80817, t80821, t80825, t80827)
}
