//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1394/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1394<F: Float>(t23476: F, t23479: F, t6722: F, t23563: F, t6740: F, t6747: F, t1014: F, t10469: F, t363: F, t23422: F, t3139: F, t10922: F, t6717: F) -> (F, F, F, F, F) {
    let t83134 = t6722 * t23476 * t23479;
    let t83138 = t6740 * t23563;
    let t83139 = t83138 * t6747;
    let t83142 = t10469 * t1014 * t363;
    let t83153 = t23422 * t3139;
    let t83157 = t6717 * t10922;
    (t83134, t83139, t83142, t83153, t83157)
}
