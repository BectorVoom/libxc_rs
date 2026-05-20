//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2896/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2896<F: Float>(t2885: F, t5737: F, t2904: F, t5769: F, t2842: F, t2844: F, t60395: F, t17423: F, t2787: F, t41831: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t48087: F, t48096: F, t48098: F) -> (F, F, F, F, F) {
    let t60407 = t5737 * t2885;
    let t60424 = t5769 * t2904;
    let t60429 = F::cast_from(0.32163958997385070134e2_f64) * t2842 * t60395 * t2844;
    let t60434 = F::new(2.0) * t2787 * t17423;
    let t60449 = F::cast_from(0.65725333333333333332e0_f64) * t48087 + F::cast_from(0.10629925925925925926e1_f64) * t47705 - F::cast_from(0.35433086419753086419e0_f64) * t47707 + F::cast_from(0.26574814814814814814e0_f64) * t47709 + F::cast_from(0.13287407407407407407e0_f64) * t47711 + F::cast_from(0.22145679012345679012e0_f64) * t47713 - F::cast_from(0.79724444444444444444e0_f64) * t47715 - F::cast_from(0.39862222222222222222e0_f64) * t47717 - F::cast_from(0.79724444444444444443e0_f64) * t47724 + F::cast_from(0.18257037037037037037e0_f64) * t41831 - F::cast_from(0.36514074074074074074e0_f64) * t48096 + F::cast_from(0.10954222222222222222e0_f64) * t48098 - F::cast_from(0.5314962962962962963e0_f64) * t47730 + F::cast_from(0.19931111111111111111e0_f64) * t47732;
    (t60407, t60424, t60429, t60434, t60449)
}
