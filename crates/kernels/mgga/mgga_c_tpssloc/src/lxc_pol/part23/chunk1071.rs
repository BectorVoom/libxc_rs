//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1071/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1071<F: Float>(t41961: F, t275: F, t2790: F, t2840: F, t41654: F, t2843: F, t2928: F, t315: F, t2931: F, t2859: F, t2884: F, t302: F, t2887: F, t271: F, t2770: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41962 = 0.13490888888888888889e1 * t41961;
    let t42028 = t275 / t2840 / t2790;
    let t42086 = 0.31003950617283950618e1 * t41654;
    let t42087 = 0.13388493827160493828e1 * t41961;
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    let t42102 = 1.0 / t42101;
    let t42109 = t2928 * t2928;
    let t42110 = 1.0 / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    let t42113 = 1.0 / t42112;
    let t42154 = t302 / t2884 / t2859;
    let t42212 = 0.5356037037037037037e1 * t41654;
    let t42213 = 0.16979925925925925926e1 * t41961;
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    let t42227 = t2887 * t2887;
    let t42228 = 1.0 / t42227;
    let t42245 = 0.17757530864197530864e0 * t41654;
    let t42308 = 1.0 / t271 / t2770;
    (t41962, t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42113, t42154, t42212, t42213, t42226, t42228, t42245, t42308)
}
