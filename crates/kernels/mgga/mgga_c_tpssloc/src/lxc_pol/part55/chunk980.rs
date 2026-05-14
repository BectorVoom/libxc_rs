//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 980/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk980<F: Float>(t113: F, t1266: F, t2114: F, t2165: F, t30993: F, t30995: F, t31034: F, t31038: F, t31046: F, t31050: F, t31833: F, t31834: F, t31835: F, t31838: F, t32572: F, t32595: F, t32605: F, t510: F, t650: F, t652: F, t7264: F, t7266: F, t7271: F, t7408: F, t8329: F, t8860: F, t8913: F) -> (F,) {
    let t32608 = -t113 * t32572 - t1266 * t8860 - 2.0 * t2114 * t7408 - 2.0 * t2165 * t7264 - t32595 * t510 - 2.0 * t32605 * t652 - t650 * t8913 - 4.0 * t7266 * t7271 - t30993 - t30995 - t31034 - t31038 + t31046 + t31050 + 2.0 * t31833 + 2.0 * t31834 - 2.0 * t31835 - 4.0 * t31838 - t8329;
    (t32608,)
}
