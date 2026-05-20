//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2027/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2027<F: Float>(t102647: F, t102663: F, t102679: F, t102694: F, t102705: F, t102715: F, t102732: F, t102746: F, t544: F, t553: F, t6378: F, t7211: F, t90993: F, t91000: F, t91002: F, t93618: F, t97119: F, t97124: F, t97129: F, t97135: F, t97137: F, t97142: F, t97146: F, t97148: F, t97152: F, t97158: F, t97161: F) -> (F, F) {
    let t102749 = t102647 + t102663 + t102679 + t102694 + t102705 + t102715 + t102732 + t102746;
    let t102765 = -F::cast_from(0.3289868133696452873e-1_f64) * t90993 + F::cast_from(0.6579736267392905746e-1_f64) * t97119 + t544 * t553 * t102749 - F::cast_from(0.15352717957250113407e0_f64) * t97124 + t6378 * t7211 - F::cast_from(0.3289868133696452873e-1_f64) * t97129 + F::cast_from(0.19739208802178717238e0_f64) * t97135 + F::cast_from(0.76763589786250567037e-1_f64) * t97137 + F::cast_from(0.82246703342411321825e-2_f64) * t97142 - F::cast_from(0.25587863262083522345e0_f64) * t91000 - F::cast_from(0.46058153871750340221e0_f64) * t91002 - F::cast_from(0.13159472534785811492e0_f64) * t97146 + F::cast_from(0.38381794893125283518e-1_f64) * t97148 + F::cast_from(0.3289868133696452873e-1_f64) * t97152 + t93618 + F::cast_from(0.9869604401089358619e-1_f64) * t97158 - F::cast_from(0.49348022005446793095e-1_f64) * t97161;
    (t102749, t102765)
}
