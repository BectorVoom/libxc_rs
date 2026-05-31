//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3068/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3068<F: Float>(t449: F, t63665: F, t63679: F, t63692: F, t63706: F, t11275: F, t11277: F, t3265: F, t6020: F, t11297: F, t11350: F, t11352: F, t11356: F, t11361: F, t11415: F, t1155: F, t15117: F, t15153: F, t15156: F, t15207: F, t18606: F, t18609: F, t18612: F, t18616: F, t18647: F, t18650: F, t18651: F, t18786: F, t3333: F, t3351: F, t3357: F, t3376: F, t44172: F, t44177: F, t44179: F, t44202: F, t44205: F, t4802: F, t4823: F, t51730: F, t6036: F, t6052: F, t6069: F, t6085: F) -> (F, F, F) {
    let t63709 = (t63665 + t63679 + t63692 + t63706) * t449;
    let t63714 = F::cast_from(0.51726012919273400301e3_f64) * t11275 * t6020 * t11277 * t3265;
    let t63715 = -F::cast_from(0.11696447245269292414e1_f64) * t44202 * t6069 + F::cast_from(0.5848223622634646207e0_f64) * t11356 * t6085 - F::cast_from(0.46785788981077169656e1_f64) * t11297 * t18606 - F::cast_from(0.2077903092681775651e3_f64) * t44205 * t18609 - F::cast_from(0.23392894490538584828e1_f64) * t11297 * t18612 + F::cast_from(0.34631718211362927518e2_f64) * t11361 * t18616 - F::cast_from(0.23392894490538584828e1_f64) * t3376 * t18786 * t1155 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t6052 * t11352 * t3333 + F::cast_from(0.12865583598954028054e3_f64) * t11415 * t18647 + F::cast_from(0.64327917994770140268e2_f64) * t3357 * t4823 * t15117 + F::cast_from(0.4138081033541872024e4_f64) * t44172 * t18651 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t18650 * t3351 + F::cast_from(0.19964560303604640732e6_f64) * t44177 * t6036 * t44179 * t3333 - F::cast_from(8.0_f64) * t51730 * t4802 - F::cast_from(8.0_f64) * t15207 * t15153 - F::cast_from(4.0_f64) * t15207 * t15156 - F::cast_from(0.19751673498613801407e-1_f64) * t63709 - t63714;
    (t63709, t63714, t63715)
}
