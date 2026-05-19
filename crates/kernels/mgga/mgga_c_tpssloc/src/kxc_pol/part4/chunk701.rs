//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 701/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk701<F: Float>(t1137: F, t4819: F, t1682: F, t3359: F, t1136: F, t3238: F, t3363: F, t4721: F, t4726: F, t4731: F, t4735: F, t449: F) -> (F, F, F, F, F) {
    let t4820 = t4819 * t1137;
    let t4823 = t1682 * t3359;
    let t4824 = t4823 * t1136;
    let t4832 = t3363 - F::cast_from(0.30902777777777777778e-2_f64) * t3238 - F::cast_from(0.30902777777777777778e-2_f64) * t4721 - F::cast_from(0.61805555555555555555e-2_f64) * t4726 + F::cast_from(0.18541666666666666667e-1_f64) * t4731 + F::cast_from(0.92708333333333333333e-2_f64) * t4735;
    let t4833 = t4832 * t449;
    (t4820, t4823, t4824, t4832, t4833)
}
