//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 816/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk816<F: Float>(t2517: F, t718: F, t2475: F, t723: F, t159: F, t2461: F, t730: F, t167: F, t2478: F, t164: F, t2479: F, t9689: F, t9692: F, t9695: F, t9698: F, t9702: F, t9704: F, t9706: F, t9709: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9726 = t718 * t2517;
    let t9727 = F::new(3.0) * t9726;
    let t9729 = F::new(1.0) / t2475 / t723;
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = F::new(1.0) / t2478 / t167;
    let t9734 = t9731 * t9733;
    let t9738 = F::new(1.0) / t2475 / t164;
    let t9739 = t159 * t9738;
    let t9740 = t9731 * t2479;
    let t9751 = -F::new(0.47063e1) * t9689 + F::cast_from(0.31375333333333333334e1_f64) * t9692 - F::cast_from(0.36604555555555555556e1_f64) * t9695 - F::cast_from(0.16068111111111111111e1_f64) * t9698 + F::cast_from(0.28051666666666666666e0_f64) * t9702 - F::cast_from(0.56103333333333333332e0_f64) * t9704 - F::cast_from(0.6545388888888888889e0_f64) * t9706 - F::cast_from(0.46308888888888888888e0_f64) * t9709;
    (t9727, t9729, t9730, t9731, t9733, t9734, t9738, t9739, t9740, t9751)
}
