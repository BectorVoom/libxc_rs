//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 906/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk906<F: Float>(t10553: F, t10602: F, t942: F, t951: F, t959: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F) -> (F, F, F) {
    let t10603 = t10553 + t10602;
    let t10605 = t942 * t10603 * t951;
    let t10607 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t10605;
    let t10608 = F::cast_from(0.28842592592592592592e-1_f64) * t10544;
    let t10619 = -t10608 - F::cast_from(0.12361111111111111111e-1_f64) * t10556 + F::cast_from(0.61805555555555555556e-2_f64) * t10558 - F::cast_from(0.18541666666666666667e-1_f64) * t10560 + F::cast_from(0.92708333333333333334e-2_f64) * t10562 - F::cast_from(0.10300925925925925926e-1_f64) * t10566 + F::cast_from(0.37083333333333333333e-1_f64) * t10569 - F::cast_from(0.18541666666666666666e-1_f64) * t10530 - F::cast_from(0.55625000000000000001e-1_f64) * t10572 + F::cast_from(0.55625000000000000001e-1_f64) * t10538 - F::cast_from(0.92708333333333333333e-2_f64) * t10575;
    (t10603, t10607, t10619)
}
