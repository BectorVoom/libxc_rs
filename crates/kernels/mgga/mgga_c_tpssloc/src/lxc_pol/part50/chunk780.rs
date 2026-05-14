//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 780/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk780<F: Float>(t83: F, t84: F, t85: F, t24: F, t1891: F, t67: F, t246: F, t856: F, t68: F, t261: F, t2751: F) -> (F, F, F, F, F, F, F) {
    let t9238 = 1.0 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t10108 = t856 * t856;
    let t10109 = 1.0 / t10108;
    let t10110 = t68 * t10109;
    let t10143 = 1.0 / t2751 / t261;
    (t9238, t9239, t9646, t10108, t10109, t10110, t10143)
}
