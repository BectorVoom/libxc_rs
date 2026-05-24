//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 514/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk514<F: Float>(t209: F, t469: F, t5666: F, t6: F, t219: F, t4467: F, t1144: F, t1516: F, t4462: F, t612: F, t1195: F, t1467: F, t1500: F, t4505: F, t4560: F, t467: F, t488: F, t5571: F, t5574: F, t5579: F, t5585: F, t5587: F, t5592: F, t5597: F, t5602: F, t5607: F, t5611: F, t5616: F, t5621: F, t5625: F, t5630: F, t5633: F, t5636: F) -> F {
    let t5669 = t469 * t6 * t5666 * t209;
    let t5672 = t4467 * t219;
    let t5674 = t5672 * t1516 * t1144;
    let t5677 = t4462 * t612;
    let t5679 = -t5571 + F::cast_from(0.54879112805223954488e-1_f64) * t1195 * t5574 - F::cast_from(0.27439556402611977244e-1_f64) * t1500 * t5579 - t5585 - F::cast_from(0.16463733841567186346e0_f64) * t4505 * t5587 + F::cast_from(0.10975822561044790898e0_f64) * t1195 * t5592 + F::cast_from(0.54879112805223954488e-1_f64) * t1195 * t5597 - F::cast_from(0.27439556402611977244e-1_f64) * t1500 * t5602 + F::cast_from(0.10975822561044790898e0_f64) * t1195 * t5607 + F::cast_from(0.54879112805223954488e-1_f64) * t1195 * t5611 - F::cast_from(0.54879112805223954488e-1_f64) * t1500 * t5616 + F::cast_from(0.10975822561044790898e0_f64) * t1467 * t5621 + F::cast_from(0.54879112805223954488e-1_f64) * t1467 * t5625 - F::cast_from(0.25610252642437845428e0_f64) * t4560 + F::cast_from(0.16463733841567186346e0_f64) * t488 * t5630 - F::cast_from(0.76830757927313536283e0_f64) * t5633 + t5636 - F::cast_from(0.27439556402611977244e-1_f64) * t467 * t5669 - F::cast_from(0.65854935366268745384e0_f64) * t488 * t5674 - F::cast_from(0.42683754404063075713e0_f64) * t5677;
    t5679
}
