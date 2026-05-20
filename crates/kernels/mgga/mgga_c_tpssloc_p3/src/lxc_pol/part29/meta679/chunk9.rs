//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2285/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2285<F: Float>(t27381: F, t7294: F, t1715: F, t3475: F, t1186: F, t11928: F, t1238: F, t15802: F, t1760: F, t2155: F, t24589: F, t24597: F, t24603: F, t24615: F, t24616: F, t24867: F, t24897: F, t27406: F, t27437: F, t27549: F, t27751: F, t27761: F, t27775: F, t27799: F, t3477: F, t3593: F, t3598: F, t4723: F, t4945: F, t52386: F, t7283: F, t7300: F, t8010: F, t8088: F, t86403: F, t86415: F, t94369: F) -> (F, F) {
    let t94584 = t7294 * t27381;
    let t94588 = t1715 * t3475;
    let t94605 = F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86415 * t27437 - F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t94369 * t4723 * t24603 - F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t86403 * t27775 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t7300 * t24615 * t15802 - t11928 * t8088 - F::cast_from(0.9747757433174675179e-2_f64) * t27406 * t24597 + F::new(4.0) * t3593 * t27761 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t94584 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t94588 * t27799 - F::new(6.0) * t4945 * t24897 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27751 * t24616 - t52386 * t2155 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3477 * t8010 + F::new(2.0) * t1238 * t3598 * t24867 * t1760;
    (t94588, t94605)
}
