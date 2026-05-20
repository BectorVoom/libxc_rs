//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2019/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2019<F: Float>(t1016: F, t3034: F, t1081: F, t2752: F, t608: F, t9239: F, t835: F, t531: F, t6995: F, t22573: F, t6875: F, t111: F, t7415: F) -> (F, F, F, F, F, F, F) {
    let t82985 = F::new(1.0) / t3034 / t1016;
    let t83555 = t2752 * t1081;
    let t83717 = t9239 * t608;
    let t83803 = F::new(1232.0) / F::new(27.0) * t835;
    let t83859 = t531 * t6995;
    let t83886 = t6875 * t22573;
    let t85416 = t7415 * t111;
    (t82985, t83555, t83717, t83803, t83859, t83886, t85416)
}
