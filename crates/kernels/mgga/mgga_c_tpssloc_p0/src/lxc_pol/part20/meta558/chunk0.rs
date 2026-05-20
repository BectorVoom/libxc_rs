//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2114/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2114<F: Float>(t10770: F, t919: F, t2897: F, t2904: F, t10701: F, t888: F, t275: F, t2790: F, t2840: F, t41654: F, t41961: F, t2843: F) -> (F, F, F, F, F, F, F, F) {
    let t41984 = t919 * t10770;
    let t42020 = t2897 * t2904;
    let t42023 = t888 * t10701;
    let t42028 = t275 / t2840 / t2790;
    let t42086 = F::cast_from(0.31003950617283950618e1_f64) * t41654;
    let t42087 = F::cast_from(0.13388493827160493828e1_f64) * t41961;
    let t42098 = t2840 * t2840;
    let t42100 = t275 / t42098;
    let t42101 = t2843 * t2843;
    (t41984, t42020, t42023, t42028, t42086, t42087, t42100, t42101)
}
