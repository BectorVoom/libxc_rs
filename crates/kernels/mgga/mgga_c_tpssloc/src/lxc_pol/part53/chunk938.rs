//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 938/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk938<F: Float>(t226: F, t235: F, t2690: F, t8344: F, t23139: F, t8339: F, t79: F, t8306: F, t22779: F, t31162: F, t22817: F, t794: F, t8462: F) -> (F, F, F, F, F) {
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112855 = t23139 * t8339;
    let t113875 = t8306 * t79;
    let t113966 = t22779 * t31162;
    let t113981 = t22817 * t794 * t8462;
    (t112850, t112855, t113875, t113966, t113981)
}
