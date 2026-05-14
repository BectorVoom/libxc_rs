//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1076/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1076<F: Float>(t7000: F, t8690: F, t6535: F, t7266: F, t113: F, t1869: F, t30993: F, t30995: F, t31034: F, t31038: F, t31039: F, t31041: F, t31829: F, t31833: F, t31834: F, t7408: F, t8329: F) -> (F,) {
    let t31835 = t8690 * t7000;
    let t31838 = t7266 * t6535;
    let t31840 = -t113 * t31829 - t1869 * t7408 - t30993 - t30995 - t31034 - t31038 + 3.0 * t31039 - t31041 + t31833 + t31834 - t31835 - 2.0 * t31838 - t8329;
    (t31840,)
}
