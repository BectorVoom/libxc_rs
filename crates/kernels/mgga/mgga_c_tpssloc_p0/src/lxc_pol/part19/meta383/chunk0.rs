//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1431/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1431<F: Float>(t3368: F, t3375: F, t11292: F, t1143: F, t3324: F, t3331: F, t1124: F, t11419: F, t11282: F, t43689: F, t440: F, t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F, t43754: F) -> (F, F, F, F, F, F, F) {
    let t44202 = t3368 * t3375;
    let t44205 = t1143 * t11292;
    let t44211 = t3324 * t3331;
    let t44214 = t1124 * t11419;
    let t44220 = t1143 * t11282;
    let t44223 = t440 * t43689;
    let t44243 = -F::cast_from(0.125034e1_f64) * t43713 - F::cast_from(0.13892666666666666667e0_f64) * t43717 + F::cast_from(0.375102e1_f64) * t43721 + F::cast_from(0.83356e0_f64) * t43725 + F::cast_from(0.13772666666666666667e1_f64) * t43727 - F::cast_from(0.41318e1_f64) * t43729 + F::cast_from(0.34431666666666666667e1_f64) * t43734 - F::cast_from(0.123954e2_f64) * t43737 - F::cast_from(0.13772666666666666667e1_f64) * t43740 + F::cast_from(0.185931e2_f64) * t43743 + F::cast_from(0.41318e1_f64) * t43746 - F::cast_from(0.91817777777777777776e0_f64) * t43748 - F::cast_from(0.76514814814814814814e0_f64) * t43750 - F::cast_from(0.104195e0_f64) * t43754;
    (t44202, t44205, t44211, t44214, t44220, t44223, t44243)
}
