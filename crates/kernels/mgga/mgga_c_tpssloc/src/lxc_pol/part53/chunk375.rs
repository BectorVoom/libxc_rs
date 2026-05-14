//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 375/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk375<F: Float>(t113: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t510: F, t574: F, t652: F) -> (F,) {
    let t2098 = -t113 * t2075 + t1983 * t2096 - t2036 * t510 - 2.0 * t2040 * t652 + t2079 * t574;
    (t2098,)
}
