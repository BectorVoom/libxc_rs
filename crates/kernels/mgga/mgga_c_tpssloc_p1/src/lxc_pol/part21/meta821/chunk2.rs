//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2888/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888<F: Float>(t136: F, t59696: F, t908: F, t2826: F, t59742: F, t47787: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F) -> (F, F, F) {
    let t60282 = t136 * t908 * t59696;
    let t60296 = t136 * t2826 * t59742;
    let t60300 = F::cast_from(0.16557e0_f64) * t60282 - F::cast_from(0.80513333333333333333e0_f64) * t59700 + F::cast_from(0.26837777777777777778e0_f64) * t59702 + F::cast_from(0.22364814814814814814e0_f64) * t59704 - F::cast_from(0.33547222222222222222e0_f64) * t59708 - F::cast_from(0.89459259259259259259e0_f64) * t59713 + F::cast_from(0.12077e1_f64) * t59717 - F::cast_from(0.40256666666666666666e0_f64) * t59721 - F::cast_from(0.33547222222222222222e0_f64) * t59727 + F::cast_from(0.12077e1_f64) * t59732 - F::cast_from(0.13418888888888888889e1_f64) * t59735 + F::cast_from(0.48307999999999999999e1_f64) * t59738 + F::cast_from(0.16557e0_f64) * t60296 + F::cast_from(0.62621481481481481484e0_f64) * t47787 + F::cast_from(0.12077e1_f64) * t59744;
    (t60282, t60296, t60300)
}
