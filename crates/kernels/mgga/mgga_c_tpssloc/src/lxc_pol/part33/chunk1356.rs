//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1356/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1356<F: Float>(t100431: F, t100436: F, t1058: F, t1060: F, t106058: F, t14608: F, t1599: F, t1945: F, t1949: F, t21118: F, t21130: F, t21134: F, t21446: F, t21594: F, t23696: F, t28597: F, t28630: F, t3200: F, t3201: F, t5838: F, t6687: F, t6784: F, t6785: F, t7614: F, t82676: F) -> F {
    let t106407 = t1058 * t1945 * t21594 * t1060 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t1599 * t28630 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t5838 * t7614 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t21446 * t1949 - F::new(3.0) * t14608 * t28597 - F::cast_from(0.82246703342411321826e-2_f64) * t100431 + F::cast_from(0.27415567780803773942e-2_f64) * t100436 + F::cast_from(0.8529287754027840782e-2_f64) * t6687 * t82676 * t6785 * t21130 - F::cast_from(0.21932454224643019154e-1_f64) * t6687 * t23696 * t6785 * t21118 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t6784 * t6785 * t21134 - F::new(3.0) * t3200 * t106058 * t3201;
    t106407
}
