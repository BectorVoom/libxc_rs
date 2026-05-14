//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1082/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1082<F: Float>(t31918: F, t652: F, t31247: F, t31249: F, t31250: F, t31880: F, t31898: F, t31900: F, t31902: F, t31904: F, t31906: F, t31909: F, t31913: F, t31916: F, t6539: F, t672: F, t7266: F) -> (F,) {
    let t31919 = t652 * t31918;
    let t31921 = -2.0 * t31880 * t672 - 2.0 * t31913 * t652 - 2.0 * t6539 * t7266 + t31247 - t31249 + t31250 - 2.0 * t31898 - 2.0 * t31900 - 2.0 * t31902 - 2.0 * t31904 - 2.0 * t31906 - 2.0 * t31909 + 3.0 * t31916 - 2.0 * t31919;
    (t31921,)
}
