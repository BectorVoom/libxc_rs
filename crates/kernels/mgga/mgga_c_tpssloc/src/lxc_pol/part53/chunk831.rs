//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 831/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk831<F: Float>(t2039: F, t649: F, t7056: F, t89: F, t88: F, t1441: F, t3701: F, t8807: F, t1390: F, t8803: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34682 = t649 * t2039;
    let t34685 = t89 * t7056;
    let t34707 = t88 * t7056;
    let t35233 = t1441 * t2039;
    let t38018 = t8807 * t3701;
    let t38024 = t8803 * t1390;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0 / t12019 / t566;
    (t34682, t34685, t34707, t35233, t38018, t38024, t39054, t39063, t40590)
}
