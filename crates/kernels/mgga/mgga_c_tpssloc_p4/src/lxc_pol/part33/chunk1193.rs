//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1193/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1193<F: Float>(t1949: F, t5844: F, t5838: F, t1599: F, t7614: F, t23678: F, t5928: F, t23677: F, t23604: F, t23603: F, t28596: F, t3188: F) -> (F, F, F, F, F, F, F, F) {
    let t28657 = t5844 * t1949;
    let t28660 = t5838 * t1949;
    let t28663 = t1599 * t7614;
    let t28666 = t5928 * t23678;
    let t28667 = t23677 * t28666;
    let t28670 = t5928 * t23604;
    let t28671 = t23603 * t28670;
    let t28674 = t28596 * t3188;
    (t28657, t28660, t28663, t28666, t28667, t28670, t28671, t28674)
}
