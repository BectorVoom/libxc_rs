//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 607/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk607<F: Float>(t2039: F, t7266: F, t8446: F, t8598: F, t8603: F, t8829: F, t2036: F, t2040: F, t2075: F, t2096: F, t2114: F, t2165: F, t510: F, t574: F, t652: F, t8329: F, t8522: F, t8528: F, t8535: F, t8596: F, t8608: F, t8642: F, t8645: F, t8690: F, t8835: F) -> (F, F) {
    let t8840 = 2.0 * t2039 * t7266 + t8446 + t8598 + t8603 + t8829;
    let t8843 = -t2036 * t2165 - 2.0 * t2040 * t7266 - t2075 * t2114 + t2096 * t8690 - t510 * t8829 + t574 * t8840 - 2.0 * t652 * t8835 - t8329 - t8522 - t8528 - t8535 - t8596 + t8608 + t8642 - t8645;
    (t8840, t8843)
}
