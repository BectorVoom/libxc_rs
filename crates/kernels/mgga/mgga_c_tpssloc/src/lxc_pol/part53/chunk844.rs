//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 844/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk844<F: Float>(t7106: F, t857: F, t23030: F, t31405: F, t31315: F, t6562: F, t794: F, t23012: F, t8548: F, t214: F, t7084: F, t31329: F, t6547: F, t31319: F, t23168: F, t31367: F) -> (F, F, F, F, F, F, F, F) {
    let t114797 = t857 * t7106;
    let t114814 = t23030 * t31405;
    let t114827 = t6562 * t794 * t31315;
    let t114864 = t23012 * t8548;
    let t114866 = t214 * t7084;
    let t114882 = t6547 * t31329;
    let t114891 = t23030 * t31319;
    let t114900 = t23168 * t31367;
    (t114797, t114814, t114827, t114864, t114866, t114882, t114891, t114900)
}
