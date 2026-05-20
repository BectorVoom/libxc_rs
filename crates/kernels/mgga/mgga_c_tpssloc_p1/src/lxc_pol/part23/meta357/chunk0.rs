//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1154/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1154<F: Float>(t273: F, t41654: F, t242: F, t281: F, t283: F, t275: F, t2790: F, t2840: F, t2843: F, t2928: F, t315: F, t2931: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41942 = F::powf(t273, -F::new(0.25e1));
    let t41959 = F::cast_from(0.31310740740740740741e1_f64) * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = F::cast_from(0.13490888888888888889e1_f64) * t41961;
    let t42028 = t275 / t2840 / t2790;
    let t42086 = F::cast_from(0.31003950617283950618e1_f64) * t41654;
    let t42087 = F::cast_from(0.13388493827160493828e1_f64) * t41961;
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    let t42102 = F::new(1.0) / t42101;
    let t42109 = t2928 * t2928;
    let t42110 = F::new(1.0) / t42109;
    let t42111 = t315 * t42110;
    let t42112 = t2931 * t2931;
    (t41942, t41959, t41961, t41962, t42028, t42086, t42087, t42100, t42102, t42110, t42111, t42112)
}
