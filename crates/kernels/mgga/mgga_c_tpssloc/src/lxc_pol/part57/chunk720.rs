//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 720/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk720<F: Float>(t23678: F, t5928: F, t23677: F, t23604: F, t23603: F, t28596: F, t3188: F, t1058: F, t1610: F, t1953: F, t23327: F, t23601: F, t23633: F, t25530: F, t25563: F, t28638: F, t28642: F, t28648: F, t28653: F, t28657: F, t28660: F, t28663: F, t3186: F, t5903: F, t6687: F, t7622: F) -> (F,) {
    let t28666 = t5928 * t23678;
    let t28667 = t23677 * t28666;
    let t28670 = t5928 * t23604;
    let t28671 = t23603 * t28670;
    let t28674 = t28596 * t3188;
    let t28677 = 0.36554090374405031923e-2 * t6687 * t28638 + t1058 * t28642 + 0.54831135561607547884e-2 * t25530 + t5903 * t1953 + 2.0 * t1610 * t7622 - 0.54831135561607547884e-2 * t23327 * t28648 + 0.54831135561607547884e-2 * t23633 * t28653 + 0.18277045187202515961e-2 * t25563 - 0.82246703342411321825e-2 * t6687 * t28657 - 0.82246703342411321825e-2 * t6687 * t28660 - 0.16449340668482264365e-1 * t6687 * t28663 + 0.16449340668482264365e-1 * t23601 * t28667 - 0.82246703342411321825e-2 * t23601 * t28671 + 2.0 * t3186 * t28674;
    (t28677,)
}
