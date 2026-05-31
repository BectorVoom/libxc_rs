//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1358/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1358<F: Float>(t106355: F, t1603: F, t17588: F, t1922: F, t21138: F, t21446: F, t21458: F, t21663: F, t25784: F, t28593: F, t28679: F, t349: F, t388: F, t4557: F, t5838: F, t6687: F, t6689: F, t6690: F, t6771: F, t7561: F, t7625: F, t88882: F, t99439: F, t99864: F) -> F {
    let t106460 = -t6771 * t21663 + F::cast_from(0.54831135561607547884e-2_f64) * t88882 - F::cast_from(0.82246703342411321826e-2_f64) * t99439 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6689 * t6690 * t21138 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t21446 * t1922 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t5838 * t7561 - F::cast_from(3.0_f64) * t4557 * t28679 - F::cast_from(6.0_f64) * t17588 * t7625 + F::cast_from(3.0_f64) * t1603 * t28593 * t388 + t349 * t106355 * t388 - F::cast_from(0.16449340668482264365e-1_f64) * t99864 + F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t5838 * t25784 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t21458 * t1922;
    t106460
}
